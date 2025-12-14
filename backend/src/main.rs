use std::{env, net::SocketAddr};

use axum::{
  Router,
  routing::{get, post},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{
  cors::{Any, CorsLayer},
  trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod services;
mod utils;

#[derive(Clone)]
pub struct AppState {
  pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  tracing_subscriber::registry()
    .with(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "harmony_backend=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to database.");

  sqlx::migrate!("./migrations")
    .run(&db)
    .await
    .expect("Failed to run migrations.");

  tracing::info!("Database migrations completed.");

  let state = AppState { db };

  let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

  let app = Router::new()
    .route("/", get(root_handler))
    .route("/health", get(health_check))
    .route("/api/auth/register", post(handlers::auth::register))
    .route("/api/auth/login", post(handlers::auth::login))
    .layer(cors)
    .layer(TraceLayer::new_for_http())
    .with_state(state);

  let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
  let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
  let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

  tracing::info!("Server listening on {}", addr);

  let listener = TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}

async fn root_handler() -> &'static str {
  "Harmony API Server"
}

async fn health_check() -> &'static str {
  "OK"
}
