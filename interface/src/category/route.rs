use super::handler::list_paginated_categories;
use axum::{routing::get, Router};
use infra::state::AppState;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
  Router::new().route("/categories.list", get(list_paginated_categories))
}
