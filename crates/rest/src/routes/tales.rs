use crate::{
    extractors::user::UserExtractor,
    model::tales::{CreateTaleRequest, TalePath, UpdateTaleRequest},
    ApiResponse, AppState, ResourceLink,
};
use actix_web::web::{Data, Json};
use cqrs::command_bus::{
    bus::CommandBus,
    command::{
        create_fragment::CreateFragmentCommandBuilder,
        update_fragment::{UpdateFragmentCommandBuilder, UpdateFragmentCommandError},
    },
};

pub struct TalesRouter;

impl TalesRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "tales";
    pub const SINGLE_RESOURCE_NAME: &str = "tale";

    pub async fn create<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<CreateTaleRequest>,
    ) -> ApiResponse<()> {
        let id = state.ids.new_id();
        let command = CreateFragmentCommandBuilder::default()
            .fragment_id(id)
            .content(payload.content())
            .build()
            .unwrap();

        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, Some(ResourceLink::Tale(id))),
            Err(e) => ApiResponse::InternalServerError(e.into()),
        }
    }

    pub async fn update<C: CommandBus>(
        state: Data<AppState<C>>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<UpdateTaleRequest>,
        path: TalePath,
    ) -> ApiResponse<()> {
        let command = UpdateFragmentCommandBuilder::default()
            .fragment_id(path.into_inner())
            .content(payload.content().clone())
            .end(payload.end())
            .build()
            .unwrap();

        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Ok(None),
            Err(e) => match e {
                cqrs::command_bus::error::CommandBusError::UpdateFragmentCommand(e) => match e {
                    UpdateFragmentCommandError::FragmentNotFound(_) => {
                        ApiResponse::NotFound("Tale not found")
                    }
                    UpdateFragmentCommandError::UserWithoutPermission(_) => todo!(),
                    UpdateFragmentCommandError::NonEditableFragment(_) => todo!(),
                    UpdateFragmentCommandError::NonEndabledFragment(_) => todo!(),
                },
                _ => ApiResponse::InternalServerError(e.into()),
            },
        }
    }
}
