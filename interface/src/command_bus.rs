use discern::{command::CommandBus, command_bus};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn new(_write_db: Arc<DatabaseConnection>) -> CommandBus {
  command_bus! {}
}
