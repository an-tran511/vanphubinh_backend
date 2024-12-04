use discern::{command::CommandBus, command_bus};
use sea_orm::DatabaseConnection;
use service::create_uom_command::{CreateUomCommand, CreateUomCommandHandler};
use service::update_uom_command::{UpdateUomCommand, UpdateUomCommandHandler};
use std::sync::Arc;

pub fn new(write_db: Arc<DatabaseConnection>) -> CommandBus {
  command_bus! {
    //Measurement service commands
    CreateUomCommand => CreateUomCommandHandler { db: write_db.clone() },
    UpdateUomCommand => UpdateUomCommandHandler { db: write_db.clone() },
  }
}
