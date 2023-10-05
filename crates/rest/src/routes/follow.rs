use super::user::UserPath;
use crate::{extractors::user::UserExtractor, ApiResponse, AppState};
use actix_web::web::Data;
use cqrs::command_bus::{
    bus::CommandBus,
    command::{follow_user::FollowUserCommandBuilder, unfollow_user::UnfollowUserCommandBuilder},
};

pub struct FollowingsRouter;

impl FollowingsRouter {
    const COLLECTION_RESOURCE_NAME: &'static str = "followings";

    pub async fn create<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        path: UserPath,
    ) -> ApiResponse<()> {
        let command = FollowUserCommandBuilder::default()
            .following_user_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    pub async fn delete<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        path: UserPath,
    ) -> ApiResponse<()> {
        let command = UnfollowUserCommandBuilder::default()
            .following_user_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}

pub struct FollowersRouter;

impl FollowersRouter {}
