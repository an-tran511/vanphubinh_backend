use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub ok: bool,
  pub code: String,
  pub source: Option<String>,
}

#[derive(Serialize)]
pub struct OkResponse {
  pub ok: bool,
}