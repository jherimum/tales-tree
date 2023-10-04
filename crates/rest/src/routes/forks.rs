use actix_web::web::{Data, Json};
use cqrs::command_bus::{bus::CommandBus, command::fork_fragment::ForkFragmentCommandBuilder};

use crate::{
    extractors::user::UserExtractor,
    model::{forks::ForkTaleRequest, tales::TalePath},
    ApiResponse, AppState, ResourceLink,
};

pub struct TaleForksRouter;

impl TaleForksRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "tale_forks";

    pub const SINGLE_RESOURCE_NAME: &str = "tale_fork";

    pub async fn create<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<ForkTaleRequest>,
        path: TalePath,
    ) -> ApiResponse<()> {
        let fork_id = state.ids.new_id();
        let command = ForkFragmentCommandBuilder::default()
            .fork_id(fork_id)
            .parent_fragment_id(path.as_ref().clone())
            .content(payload.content)
            .end(payload.end)
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => {
                ApiResponse::Created(None, Some(ResourceLink::Fork(path.into_inner(), fork_id)))
            }
            Err(e) => ApiResponse::InternalServerError(e.into()),
        }
    }
}
