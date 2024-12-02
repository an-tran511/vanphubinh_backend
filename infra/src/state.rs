use discern::command::CommandBus;
use discern::query::QueryBus;

pub struct AppState {
  pub query_bus: QueryBus,
  pub command_bus: CommandBus,
}
