use crate::uom::handler::list_paginated_uoms;
use axum::{routing::get, Router};
use infra::state::AppState;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
  Router::new().route("/uoms/list", get(list_paginated_uoms))
}
