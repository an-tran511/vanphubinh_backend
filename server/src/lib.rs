use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().unwrap();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

  let _db = match connect_db().await {
    Ok(db) => {
      tracing::info!("Connected to database");
      Arc::new(db)
    }
    Err(_) => {
      tracing::error!("Failed to connect to database");
      std::process::exit(1);
    }
  };

  let router = Router::new().layer(
    TraceLayer::new_for_http().make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
  );

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let tcp = TcpListener::bind(&addr).await.unwrap();

  tracing::debug!("Listening on http://{}", addr);
  axum::serve(tcp, router).await.unwrap();
}

pub async fn connect_db() -> Result<DatabaseConnection, Box<dyn Error>> {
  tracing::info!("Connecting to database");
  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  Ok(Database::connect(&db_url).await?)
}
