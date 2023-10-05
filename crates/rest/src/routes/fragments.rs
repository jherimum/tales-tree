use crate::{
    extractors::user::UserExtractor,
    links::ResourceLink,
    model::fragments::{CreateFragmentRequest, FragmentPath, UpdateFragmentRequest},
    response::ApiResponse,
    AppState,
};
use actix_web::web::{Data, Json};
use cqrs::command_bus::{
    command::{
        create_fragment::CreateFragmentCommandBuilder,
        publish_fragment::{PublishFragmentCommandBuilder, PublishFragmentCommandError},
        submit_fork::{SubmitForkCommandBuilder, SubmitForkCommandError},
        update_fragment::{UpdateFragmentCommandBuilder, UpdateFragmentCommandError},
    },
    error::CommandBusError,
};

pub struct FragmentsRouter;

impl FragmentsRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "fragments";
    pub const SINGLE_RESOURCE_NAME: &str = "fragment";
    pub const PUBLICATION_RESOURCE_NAME: &str = "publication";
    pub const SUBMIT_RESOURCE_NAME: &str = "submit";

    pub async fn create(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<CreateFragmentRequest>,
    ) -> ApiResponse<()> {
        let id = state.ids.new_id();
        let command = CreateFragmentCommandBuilder::default()
            .fragment_id(id)
            .content(payload.content())
            .build()
            .unwrap();

        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(None, Some(ResourceLink::Fragment(id))),
            Err(e) => ApiResponse::InternalServerError(e.into()),
        }
    }

    pub async fn delete(_: Data<AppState>, UserExtractor(_): UserExtractor) -> ApiResponse<()> {
        ApiResponse::Ok(None)
    }

    pub async fn update(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        Json(payload): Json<UpdateFragmentRequest>,
        path: FragmentPath,
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
                CommandBusError::UpdateFragmentCommand(e) => match e {
                    UpdateFragmentCommandError::FragmentNotFound(_) => todo!(),
                    UpdateFragmentCommandError::UserWithoutPermission(_) => todo!(),
                    UpdateFragmentCommandError::NonEditableFragment(_) => todo!(),
                    UpdateFragmentCommandError::NonEndabledFragment(_) => todo!(),
                },
                _ => ApiResponse::InternalServerError(e.into()),
            },
        }
    }

    pub async fn publish(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        path: FragmentPath,
    ) -> ApiResponse<()> {
        let command = PublishFragmentCommandBuilder::default()
            .fragment_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Ok(None),
            Err(e) => match e {
                CommandBusError::PublishFragmentCommand(e) => match e {
                    PublishFragmentCommandError::FragmentNotFound(_) => todo!(),
                    PublishFragmentCommandError::InvalidState(_) => todo!(),
                    PublishFragmentCommandError::Forbidden(_) => todo!(),
                },
                _ => ApiResponse::InternalServerError(e.into()),
            },
        }
    }

    pub async fn submit(
        state: Data<AppState>,
        UserExtractor(user): UserExtractor,
        path: FragmentPath,
    ) -> ApiResponse<()> {
        let command = SubmitForkCommandBuilder::default()
            .fragment_id(path.into_inner())
            .build()
            .unwrap();
        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Ok(None),
            Err(e) => match e {
                CommandBusError::SubmitForkCommand(e) => match e {
                    SubmitForkCommandError::ForkNotFound(_) => todo!(),
                    SubmitForkCommandError::Forbidden(_) => todo!(),
                    SubmitForkCommandError::InvalidState(_) => todo!(),
                },
                _ => ApiResponse::InternalServerError(e.into()),
            },
        }
    }
}
