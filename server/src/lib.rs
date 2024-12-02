use axum::Router;
use infra::database;
use std::{net::SocketAddr, sync::Arc};
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

  tracing::info!("Connecting to read database");
  let _read_db = match database::connect_db(&std::env::var("READ_DATABASE_URL").unwrap()).await {
    Ok(db) => {
      tracing::info!("Connected to read database");
      Arc::new(db)
    }
    Err(_) => {
      tracing::error!("Failed to connect to read database");
      std::process::exit(1);
    }
  };

  tracing::info!("Connecting to write database");
  let _write_db = match database::connect_db(&std::env::var("DATABASE_URL").unwrap()).await {
    Ok(db) => {
      tracing::info!("Connected to write database");
      Arc::new(db)
    }
    Err(_) => {
      tracing::error!("Failed to connect to write database");
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
