use crate::response::{ErrorResponse, OkResponse};
use axum::{response::IntoResponse, Json};

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
