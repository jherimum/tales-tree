pub mod follow;
pub mod forks;
pub mod fragments;
pub mod health;
pub mod likes;
pub mod reviews;
pub mod user;

use crate::routes::{
    follow::FollowingsRouter, forks::ForksRouter, fragments::FragmentsRouter, health::HealthRouter,
    likes::LikesRouter, reviews::ReviewsRouter,
};
use actix_web::{
    web::{self},
    Scope,
};

pub fn routes() -> Scope {
    const EMPTY_RESOURCE: &str = "";

    let users = web::scope("/v1/users").service(
        web::scope("/{user_id}")
            .service(web::resource(EMPTY_RESOURCE))
            .service(
                web::scope("/followings").service(
                    web::resource(EMPTY_RESOURCE)
                        .route(web::post().to(FollowingsRouter::create))
                        .route(web::delete().to(FollowingsRouter::delete)),
                ),
            ),
    );

    let fragments = web::scope("/v1/fragments")
        .service(
            web::resource(EMPTY_RESOURCE)
                .name(FragmentsRouter::COLLECTION_RESOURCE_NAME)
                .route(web::post().to(FragmentsRouter::create)),
        )
        .service(
            web::scope("/{fragment_id}")
                .service(
                    web::resource(EMPTY_RESOURCE)
                        .name(FragmentsRouter::SINGLE_RESOURCE_NAME)
                        .route(web::patch().to(FragmentsRouter::update))
                        .route(web::delete().to(FragmentsRouter::delete)),
                )
                .service(
                    web::scope("/publication").service(
                        web::resource(EMPTY_RESOURCE)
                            .name(FragmentsRouter::PUBLICATION_RESOURCE_NAME)
                            .route(web::post().to(FragmentsRouter::publish)),
                    ),
                )
                .service(
                    web::scope("/submit").service(
                        web::resource(EMPTY_RESOURCE)
                            .name(FragmentsRouter::PUBLICATION_RESOURCE_NAME)
                            .route(web::post().to(FragmentsRouter::submit)),
                    ),
                )
                .service(
                    web::scope("/likes").service(
                        web::resource(EMPTY_RESOURCE)
                            .name(LikesRouter::COLLECTION_RESOURCE_NAME)
                            .route(web::post().to(LikesRouter::create))
                            .route(web::delete().to(LikesRouter::delete)),
                    ),
                )
                .service(
                    web::scope("/reviews")
                        .service(
                            web::resource(EMPTY_RESOURCE)
                                .name(ReviewsRouter::COLLECTION_RESOURCE_NAME)
                                .route(web::post().to(ReviewsRouter::create)),
                        )
                        .service(web::scope("/{review_id}").service(
                            web::resource(EMPTY_RESOURCE).name(ReviewsRouter::SINGLE_RESOURCE_NAME),
                        )),
                )
                .service(
                    web::scope("/forks").service(
                        web::resource(EMPTY_RESOURCE)
                            .name(ForksRouter::COLLECTION_RESOURCE_NAME)
                            .route(web::post().to(ForksRouter::create)),
                    ),
                ),
        );

    web::scope("")
        .service(
            web::resource(HealthRouter::HEALTH_RESOURCE_NAME)
                .route(web::get().to(HealthRouter::get)),
        )
        .service(web::scope("/api").service(fragments).service(users))
}
