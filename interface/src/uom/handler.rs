use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use axum_macros::debug_handler;
use infra::state::AppState;
use service::{
  create_uom_command::{CreateUomCommand, CreateUomError},
  list_paginated_uoms_query::{ListUomsError, ListUomsQuery},
};
use std::sync::Arc;

pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(params): Query<ListUomsQuery>,
) -> Result<impl IntoResponse, ListUomsError> {
  let query = ListUomsQuery {
    page: params.page,
    per_page: params.per_page,
  };
  let uoms = state.query_bus.dispatch(query).await?;

  Ok((StatusCode::OK, Json(uoms)))
}

#[debug_handler]
pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateUomCommand>,
) -> Result<impl IntoResponse, CreateUomError> {
  let command = CreateUomCommand { name: payload.name };
  let uom = state.command_bus.dispatch(command).await?;
  Ok((StatusCode::CREATED, Json(uom)))
}
