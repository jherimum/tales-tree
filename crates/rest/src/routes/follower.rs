use crate::{extractors::user::UserExtractor, ApiResponse, AppState};
use actix_web::web::{Data, Path};
use commons::id::Id;
use cqrs::command_bus::{bus::CommandBus, command::follow_user::FollowUserCommandBuilder};

type ProfilePath = Path<Id>;

pub struct FollowingRoute;

impl FollowingRoute {
    pub async fn create<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        path: ProfilePath,
    ) -> ApiResponse<()> {
        let command = FollowUserCommandBuilder::default()
            .following_user_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, None),
            Err(e) => ApiResponse::InternalServerError(e.into()),
        }
    }
}
