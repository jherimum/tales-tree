use std::marker::PhantomData;

use actix_web::web::Data;
use cqrs::command_bus::{bus::CommandBus, command::create_fragment::CreateFragmentCommandBuilder};

use crate::AppState;

pub struct TalesRouter<C: CommandBus> {
    command_bus: PhantomData<C>,
}

impl<C: CommandBus> TalesRouter<C> {
    pub async fn create(state: Data<AppState>) -> String {
        todo!()
    }
}
