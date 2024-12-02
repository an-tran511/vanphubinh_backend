use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::state::AppState;
use service::list_paginated_uoms_query::{ListUomsError, ListUomsQuery};
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
