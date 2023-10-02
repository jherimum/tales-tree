use std::sync::Arc;

use cqrs::command_bus::bus::CommandBus;

pub mod model;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    command_bus: Arc<dyn CommandBus>,
}
