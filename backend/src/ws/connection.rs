use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

pub type Tx = mpsc::UnboundedSender<Message>;
pub type ConnectionMap = Arc<RwLock<HashMap<Uuid, Vec<Tx>>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
  // Client -> Server
  Subscribe {
    channel_id: Uuid,
  },
  Unsubscribe {
    channel_id: Uuid,
  },
  SendMessage {
    channel_id: Uuid,
    content: String,
  },

  // Server -> Client
  MessageCreated {
    id: Uuid,
    channel_id: Uuid,
    user_id: Uuid,
    username: String,
    content: String,
    created_at: String,
  },
  Error {
    message: String,
  },
  Subscribed {
    channel_id: Uuid,
  },
  Unsubscribed {
    channel_id: Uuid,
  },
}

pub struct Connection {
  pub user_id: Uuid,
  pub socket: WebSocket,
  pub connections: ConnectionMap,
  pub subscriptions: Vec<Uuid>,
}

impl Connection {
  pub fn new(user_id: Uuid, socket: WebSocket, connections: ConnectionMap) -> Self {
    Self {
      user_id,
      socket,
      connections,
      subscriptions: Vec::new(),
    }
  }

  pub async fn handle(self) {
    let (mut sender, mut receiver) = self.socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    {
      let mut conns = self.connections.write().await;
      conns
        .entry(self.user_id)
        .or_insert_with(Vec::new)
        .push(tx.clone());
    }

    tracing::info!(
      "WebSocket connection established for user: {}",
      self.user_id
    );

    let mut send_task = tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        if sender.send(msg).await.is_err() {
          break;
        }
      }
    });

    let connections_clone = self.connections.clone();
    let user_id = self.user_id;
    let mut subscriptions = self.subscriptions.clone();

    let mut recv_task = tokio::spawn(async move {
      while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
          if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
            match ws_msg {
              WsMessage::Subscribe { channel_id } => {
                if !subscriptions.contains(&channel_id) {
                  subscriptions.push(channel_id);
                  tracing::debug!("User {} subscribed to channel {}", user_id, channel_id);
                  let response = WsMessage::Subscribed { channel_id };
                  if let Ok(json) = serde_json::to_string(&response) {
                    let _ = tx.send(Message::Text(json.into()));
                  }
                }
              }
              WsMessage::Unsubscribe { channel_id } => {
                subscriptions.retain(|&id| id != channel_id);
                tracing::debug!("User {} unsubscribed from channel {}", user_id, channel_id);

                let response = WsMessage::Unsubscribed { channel_id };
                if let Ok(json) = serde_json::to_string(&response) {
                  let _ = tx.send(Message::Text(json.into()));
                }
              }
              _ => {
                tracing::warn!("Unexpected message type reeived via WebSocket");
              }
            }
          }
        } else if let Message::Close(_) = msg {
          break;
        }
      }
    });

    tokio::select! {
      _ = &mut send_task => {
        recv_task.abort();
      }
      _ = &mut recv_task => {
        send_task.abort();
      }
    }

    {
      let mut conns = connections_clone.write().await;
      if let Some(user_conns) = conns.get_mut(&user_id) {
        user_conns.retain(|conn_tx| !conn_tx.is_closed());
        if user_conns.is_empty() {
          conns.remove(&user_id);
        }
      }
    }

    tracing::info!("WebSocket connection closed for user: {}", user_id);
  }
}

// TODO: Filter broadcast to specific channels and add exclue user functionality
pub async fn broadcast_to_channel(
  connections: &ConnectionMap,
  _channel_id: Uuid,
  message: WsMessage,
  _exclude_user: Option<Uuid>,
) -> Result<(), Box<dyn std::error::Error>> {
  let json = serde_json::to_string(&message)?;
  let ws_message = Message::Text(json.into());

  let conns = connections.read().await;

  for (_user_id, user_conns) in conns.iter() {
    for tx in user_conns {
      let _ = tx.send(ws_message.clone());
    }
  }

  Ok(())
}
