use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use discern::async_trait;
use discern::command::Command;
use discern::command::CommandHandler;
use domain::uom;
use infra::util::error;
use infra::uuid::Uuid;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct UpdateUomCommand {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Error)]
pub enum UpdateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for UpdateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      UpdateUomError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code.to_string(), Some("update_uom_command".to_string())),
    )
      .into_response()
  }
}

impl Command for UpdateUomCommand {
  type Metadata = ();
  type Error = UpdateUomError;
}

pub struct UpdateUomCommandHandler {
  pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl CommandHandler<UpdateUomCommand> for UpdateUomCommandHandler {
  async fn handle(&self, command: UpdateUomCommand) -> Result<(), UpdateUomError> {
    let uom = uom::ActiveModel {
      id: Set(command.id),
      name: Set(command.name),
      created_at: NotSet,
      updated_at: NotSet,
    };
    uom.update(self.db.as_ref()).await?;
    Ok(())
  }
}
