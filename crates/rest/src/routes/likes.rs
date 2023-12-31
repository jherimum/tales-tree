use crate::{
    extractors::user::UserExtractor,
    model::fragments::FragmentPath,
    response::{ApiError, ApiResponse},
    server::AppState,
};
use actix_web::web::Data;
use cqrs::command_bus::command::{
    dislike_fragment::DislikeFragmentCommandBuilder, like_fragment::LikeFragmentCommandBuilder,
};

pub struct LikesRouter;

impl LikesRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "likes";

    pub async fn create(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        path: FragmentPath,
    ) -> ApiResponse<()> {
        let command = LikeFragmentCommandBuilder::default()
            .fragment_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, None),
            Err(e) => ApiError::InternalServerError(e.into()).into(),
        }
    }

    pub async fn delete(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        path: FragmentPath,
    ) -> ApiResponse<()> {
        let command = DislikeFragmentCommandBuilder::default()
            .fragment_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, None),
            Err(e) => ApiError::InternalServerError(e.into()).into(),
        }
    }
}
