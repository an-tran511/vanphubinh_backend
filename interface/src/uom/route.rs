use crate::uom::handler::{create_uom, list_paginated_uoms};
use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
  Router::new()
    .route("/uoms/list", get(list_paginated_uoms))
    .route("/uoms/create", post(create_uom))
}
