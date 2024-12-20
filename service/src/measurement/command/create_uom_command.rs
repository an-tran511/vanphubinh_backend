use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use discern::async_trait;
use discern::command::Command;
use discern::command::CommandHandler;
use domain::uom::ActiveModel as Uom;
use infra::util::error;
use infra::uuid::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct CreateUomCommand {
  pub name: String,
}

#[derive(Debug, Error)]
pub enum CreateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateUomError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_uom_command".to_string()))).into_response()
  }
}

#[derive(Debug, Serialize)]
pub struct CreateUomMeta {
  pub id: Uuid,
}

impl Command for CreateUomCommand {
  type Metadata = CreateUomMeta;
  type Error = CreateUomError;
}

pub struct CreateUomCommandHandler {
  pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl CommandHandler<CreateUomCommand> for CreateUomCommandHandler {
  async fn handle(&self, command: CreateUomCommand) -> Result<CreateUomMeta, CreateUomError> {
    let uom = Uom {
      name: Set(command.name.to_owned()),
      ..Default::default()
    };
    let uom = match uom.insert(self.db.as_ref()).await {
      Ok(uom) => uom,
      Err(error) => return Err(CreateUomError::InternalServerError(error)),
    };

    Ok(CreateUomMeta { id: uom.id })
  }
}
