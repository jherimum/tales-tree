use actix_web::{
    web::{self, Data},
    Route, Scope,
};
use commons::actor::ActorTrait;
use cqrs::command_bus::bus::CommandBus;

use crate::AppState;

use self::tales::TalesRouter;

pub mod tales;

pub fn routes<C: CommandBus + 'static>() -> Scope {
    web::scope("/tales")
        .service(web::resource("").route(web::post().to(TalesRouter::<C>::create)))
        .app_data(Data::new(app_state::<C>()))
}

pub fn app_state<C: CommandBus>() -> AppState {
    todo!()
}
