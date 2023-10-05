use crate::{
    extractors::user::UserExtractor,
    links::ResourceLink,
    model::{forks::ForkFragmentRequest, fragments::FragmentPath},
    response::ApiResponse,
    server::AppState,
};
use actix_web::web::{Data, Json};
use cqrs::command_bus::command::fork_fragment::ForkFragmentCommandBuilder;

pub struct ForksRouter;

impl ForksRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "fragment_forks";

    pub const SINGLE_RESOURCE_NAME: &str = "fragment_fork";

    pub async fn create(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<ForkFragmentRequest>,
        path: FragmentPath,
    ) -> ApiResponse<()> {
        let fork_id = state.ids.new_id();
        let command = ForkFragmentCommandBuilder::default()
            .fork_id(fork_id)
            .parent_fragment_id(path.into_inner())
            .content(payload.content)
            .end(payload.end)
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, Some(ResourceLink::Fragment(fork_id))),
            Err(e) => match e {
                _ => ApiResponse::InternalServerError(Box::new(e)),
            },
        }
    }
}
