use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use discern::async_trait;
use discern::query::Query;
use discern::query::QueryHandler;
use domain::uom::Entity as Uom;
use domain::uom::UomDTO;
use infra::response::ErrorResponse;
use infra::uuid::Uuid;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindUomByIdQuery {
  pub id: Uuid,
}

#[derive(Error, Debug)]
pub enum FindUomByIdError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
  #[error("not_found")]
  NotFound,
}

impl IntoResponse for FindUomByIdError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindUomByIdError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
      FindUomByIdError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
    };
    (
      status,
      Json(ErrorResponse {
        ok: false,
        code,
        source: Some("find_uom_by_id_query".to_string()),
      }),
    )
      .into_response()
  }
}

impl Query for FindUomByIdQuery {
  type Output = Option<UomDTO>;
  type Error = FindUomByIdError;
}

pub struct FindUomByIdQueryHandler {
  pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl QueryHandler<FindUomByIdQuery> for FindUomByIdQueryHandler {
  async fn handle(&self, query: FindUomByIdQuery) -> Result<Option<UomDTO>, FindUomByIdError> {
    let uom = Uom::find_by_id(query.id)
      .into_partial_model::<UomDTO>()
      .one(self.db.as_ref())
      .await;
    match uom {
      Ok(Some(uom)) => Ok(Some(uom)),
      Ok(None) => Err(FindUomByIdError::NotFound),
      Err(e) => Err(FindUomByIdError::InternalServerError(e)),
    }
  }
}
