use std::sync::Arc;

use commons::id::IdGenerator;
use cqrs::command_bus::bus::CommandBus;

pub mod extractors;
pub mod links;
pub mod model;
pub mod response;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    command_bus: Arc<CommandBus>,
    ids: Arc<dyn IdGenerator>,
}
