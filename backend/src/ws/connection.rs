use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

pub type Tx = mpsc::UnboundedSender<Message>;

// user_id -> list of connections
pub type UserConnections = Arc<RwLock<HashMap<Uuid, Vec<ConnectionHandle>>>>;

// channel_id -> set of user_ids
pub type ChannelSubscriptions = Arc<RwLock<HashMap<Uuid, HashSet<Uuid>>>>;

pub struct ConnectionHandle {
  pub tx: Tx,
  pub subscriptions: Arc<RwLock<HashSet<Uuid>>>,
}

pub struct ConnectionMap {
  pub users: UserConnections,
  pub channels: ChannelSubscriptions,
}

impl Default for ConnectionMap {
  fn default() -> Self {
    Self {
      users: Arc::new(RwLock::new(HashMap::new())),
      channels: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

impl Clone for ConnectionMap {
  fn clone(&self) -> Self {
    Self {
      users: Arc::clone(&self.users),
      channels: Arc::clone(&self.channels),
    }
  }
}

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
  pub connection_map: ConnectionMap,
  pub subscriptions: Arc<RwLock<HashSet<Uuid>>>,
}

impl Connection {
  pub fn new(user_id: Uuid, socket: WebSocket, connection_map: ConnectionMap) -> Self {
    Self {
      user_id,
      socket,
      connection_map,
      subscriptions: Arc::new(RwLock::new(HashSet::new())),
    }
  }

  pub async fn handle(self) {
    let (mut sender, mut receiver) = self.socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let connection_handle = ConnectionHandle {
      tx: tx.clone(),
      subscriptions: Arc::clone(&self.subscriptions),
    };

    {
      let mut users = self.connection_map.users.write().await;
      users
        .entry(self.user_id)
        .or_insert_with(Vec::new)
        .push(connection_handle);
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

    let connection_map_clone = self.connection_map.clone();
    let user_id = self.user_id;
    let subscriptions = Arc::clone(&self.subscriptions);

    let mut recv_task = tokio::spawn(async move {
      while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
          if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
            match ws_msg {
              WsMessage::Subscribe { channel_id } => {
                {
                  let mut subs = subscriptions.write().await;
                  subs.insert(channel_id);
                }

                {
                  let mut channels = connection_map_clone.channels.write().await;
                  channels
                    .entry(channel_id)
                    .or_insert_with(HashSet::new)
                    .insert(user_id);
                }

                tracing::debug!("User {} subscribed to channel {}", user_id, channel_id);

                let response = WsMessage::Subscribed { channel_id };
                if let Ok(json) = serde_json::to_string(&response) {
                  let _ = tx.send(Message::Text(json.into()));
                }
              }
              WsMessage::Unsubscribe { channel_id } => {
                {
                  let mut subs = subscriptions.write().await;
                  subs.remove(&channel_id);
                }

                {
                  let users = connection_map_clone.users.read().await;
                  let mut channels = connection_map_clone.channels.write().await;

                  let still_subscribed = if let Some(user_conns) = users.get(&user_id) {
                    let mut has_subscription = false;
                    for conn in user_conns {
                      let conn_subs = conn.subscriptions.read().await;
                      if conn_subs.contains(&channel_id) {
                        has_subscription = true;
                        break;
                      }
                    }
                    has_subscription
                  } else {
                    false
                  };

                  if !still_subscribed {
                    if let Some(channel_subs) = channels.get_mut(&channel_id) {
                      channel_subs.remove(&user_id);
                      if channel_subs.is_empty() {
                        channels.remove(&channel_id);
                      }
                    }
                  }
                }

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
      let subscribed_channels = {
        let subs = self.subscriptions.read().await;
        subs.clone()
      };

      let mut users = self.connection_map.users.write().await;
      if let Some(user_conns) = users.get_mut(&user_id) {
        user_conns.retain(|conn| !conn.tx.is_closed());
        if user_conns.is_empty() {
          users.remove(&user_id);
        }
      }

      let mut channels = self.connection_map.channels.write().await;
      for channel_id in subscribed_channels {
        let still_subscribed = if let Some(user_conns) = users.get(&user_id) {
          let mut has_subscription = false;
          for conn in user_conns {
            let conn_subs = conn.subscriptions.read().await;
            if conn_subs.contains(&channel_id) {
              has_subscription = true;
              break;
            }
          }
          has_subscription
        } else {
          false
        };

        if !still_subscribed {
          if let Some(channel_subs) = channels.get_mut(&channel_id) {
            channel_subs.remove(&user_id);
            if channel_subs.is_empty() {
              channels.remove(&channel_id);
            }
          }
        }
      }
    }

    tracing::info!("WebSocket connection closed for user: {}", user_id);
  }
}

pub async fn broadcast_to_channel(
  connection_map: &ConnectionMap,
  channel_id: Uuid,
  message: WsMessage,
  exclude_user: Option<Uuid>,
) -> Result<(), Box<dyn std::error::Error>> {
  let json = serde_json::to_string(&message)?;
  let ws_message = Message::Text(json.into());

  let subscribed_users = {
    let channels = connection_map.channels.read().await;
    channels
      .get(&channel_id)
      .map(|users| users.clone())
      .unwrap_or_default()
  };

  if subscribed_users.is_empty() {
    tracing::debug!("No users subscribed to channel {}", channel_id);
    return Ok(());
  }

  tracing::debug!(
    "Broadcasting to channel {} with {} subscribers",
    channel_id,
    subscribed_users.len()
  );

  let users = connection_map.users.read().await;
  let mut sent_count = 0;

  for user_id in subscribed_users {
    if let Some(excluded) = exclude_user {
      if user_id == excluded {
        continue;
      }
    }

    if let Some(user_conns) = users.get(&user_id) {
      for conn in user_conns {
        let is_subscribed = {
          let subs = conn.subscriptions.read().await;
          subs.contains(&channel_id)
        };

        if is_subscribed {
          if conn.tx.send(ws_message.clone()).is_ok() {
            sent_count += 1;
          }
        }
      }
    }
  }

  tracing::debug!(
    "Sent message to {} connections for channel {}",
    sent_count,
    channel_id
  );

  Ok(())
}
