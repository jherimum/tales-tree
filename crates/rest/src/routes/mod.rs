pub mod follow;
pub mod forks;
pub mod fragments;
pub mod likes;
pub mod reviews;
pub mod user;

use crate::AppState;
use actix_web::Scope;
use cqrs::command_bus::bus::CommandBus;

pub fn routes<C: CommandBus + 'static>(_: AppState<C>) -> Scope {
    // web::scope("/v1/me/tales")
    //     .service(
    //         web::resource("")
    //             .route(web::get().to(TalesRouter::query::<C>))
    //             .route(web::post().to(TalesRouter::create::<C>)),
    //     )
    //     .service(
    //         web::scope("/{id}")
    //             .service(
    //                 web::resource("")
    //                     .route(web::get().to(TalesRouter::get::<C>))
    //                     .route(web::patch().to(TalesRouter::update::<C>))
    //                     .route(web::delete().to(TalesRouter::delete::<C>)),
    //             )
    //             .service(
    //                 web::scope("/reviews")
    //                     .service(web::resource("").route(web::get().to(ReviewsRouter::query::<C>))),
    //             )
    //             .service(
    //                 web::scope("/forks")
    //                     .service(
    //                         web::resource("").route(web::get().to(MeTalesForksRouter::query::<C>)),
    //                     )
    //                     .service(
    //                         web::scope("/{fork_id}")
    //                             .service(
    //                                 web::resource("")
    //                                     .route(web::get().to(MeTalesForksRouter::get::<C>)),
    //                             )
    //                             .service(
    //                                 web::scope("/reviews").service(
    //                                     web::resource("")
    //                                         .route(web::get().to(ReviewsRouter::query::<C>))
    //                                         .route(web::post().to(ReviewsRouter::create::<C>)),
    //                                 ),
    //                             ),
    //                     ),
    //             ),
    //     )
    //     .app_data(Data::new(app_state))
    todo!()
}
