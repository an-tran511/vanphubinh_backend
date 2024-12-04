use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use discern::async_trait;
use discern::command::Command;
use discern::command::CommandHandler;
use domain::uom::{self, Entity as Uom};
use infra::util::{error, ok};
use infra::uuid::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
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
    let uom = Uom::find_by_id(command.id).one(self.db.as_ref()).await?;
    let mut uom: uom::ActiveModel = uom.unwrap().into();
    uom.name = Set(command.name);
    uom.update(self.db.as_ref()).await?;
    Ok(())
  }
}
