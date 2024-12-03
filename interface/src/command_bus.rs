use discern::{command::CommandBus, command_bus};
use sea_orm::DatabaseConnection;
use service::create_uom_command::{CreateUomCommand, CreateUomCommandHandler};
use std::sync::Arc;

pub fn new(_write_db: Arc<DatabaseConnection>) -> CommandBus {
  command_bus! {
    //Measurement service commands
    CreateUomCommand => CreateUomCommandHandler { db: _write_db },
  }
}
