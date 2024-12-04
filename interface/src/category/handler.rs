use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::state::AppState;
use service::list_paginated_categories_query::{ListCategoriesError, ListCategoriesQuery};
use std::sync::Arc;

pub async fn list_paginated_categories(
  State(state): State<Arc<AppState>>,
  Query(params): Query<ListCategoriesQuery>,
) -> Result<impl IntoResponse, ListCategoriesError> {
  let query = ListCategoriesQuery {
    page: params.page,
    per_page: params.per_page,
  };
  let uoms = state.query_bus.dispatch(query).await?;

  Ok((StatusCode::OK, Json(uoms)))
}
