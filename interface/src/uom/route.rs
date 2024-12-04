use super::handler::{create_uom, find_uom_by_id, list_paginated_uoms, update_uom};
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
    .route("/uoms/:id/find", get(find_uom_by_id))
    .route("/uoms/:id/update", post(update_uom))
}
