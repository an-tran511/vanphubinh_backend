use crate::response::{ErrorResponse, OkResponse};
use axum::{response::IntoResponse, response::Response, Json};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
  pub page: u64,
  pub total_pages: u64,
  pub per_page: u64,
  pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse<T> {
  pub data: Vec<T>,
  pub meta: PaginationMeta,
}

impl<T> IntoResponse for GetResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

pub fn ok() -> impl IntoResponse {
  Json(OkResponse { ok: true }).into_response()
}

pub fn error(code: String, source: Option<String>) -> impl IntoResponse {
  Json(ErrorResponse {
    ok: false,
    code,
    source,
  })
  .into_response()
}
