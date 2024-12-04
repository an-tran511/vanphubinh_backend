use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::{state::AppState, uuid::Uuid};
use service::{
  create_uom_command::{CreateUomCommand, CreateUomError},
  find_uom_by_id_query::{FindUomByIdError, FindUomByIdQuery},
  list_paginated_uoms_query::{ListUomsError, ListUomsQuery},
  update_uom_command::{UpdateUomCommand, UpdateUomError},
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

pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateUomCommand>,
) -> Result<impl IntoResponse, CreateUomError> {
  let command = CreateUomCommand { name: payload.name };
  let meta = state.command_bus.dispatch(command).await?;
  Ok((StatusCode::CREATED, Json(meta)))
}

pub async fn find_uom_by_id(
  State(state): State<Arc<AppState>>,
  Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, FindUomByIdError> {
  let query = FindUomByIdQuery { id };
  let uom = state.query_bus.dispatch(query).await?;
  Ok((StatusCode::OK, Json(uom)))
}

pub async fn update_uom(
  State(state): State<Arc<AppState>>,
  Path(id): Path<Uuid>,
  Json(payload): Json<UpdateUomCommand>,
) -> Result<impl IntoResponse, UpdateUomError> {
  let command = UpdateUomCommand {
    id,
    name: payload.name,
  };
  match state.command_bus.dispatch(command).await {
    Ok(meta) => Ok((StatusCode::OK, Json(meta))),
    Err(e) => Err(e),
  }
}
