use crate::command_bus::commands::{
    create_fragment::CreateFragmentCommandError, dislike_fragment::DislikeFragmentCommandError,
    fork_fragment::ForkFragmentCommandError, like_fragment::LikeFragmentCommandError,
    publish_fragment::PublishFragmentCommandError, review_fork::ReviewForkCommandError,
    update_fragment::UpdateFragmentCommandError,
};
use commons::actor::ActorTrait;
use storage::StorageError;

use super::commands::submit_fork::SubmitForkCommandError;

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error(transparent)]
    DislikeFragmentCommand(#[from] DislikeFragmentCommandError),

    #[error(transparent)]
    LikeFragmentCommand(#[from] LikeFragmentCommandError),

    #[error(transparent)]
    CreateFragmentCommand(#[from] CreateFragmentCommandError),

    #[error(transparent)]
    ForkFragmentCommand(#[from] ForkFragmentCommandError),

    #[error(transparent)]
    PublishFragmentCommand(#[from] PublishFragmentCommandError),

    #[error(transparent)]
    UpdateFragmentCommand(#[from] UpdateFragmentCommandError),

    #[error(transparent)]
    ReviewForkCommand(#[from] ReviewForkCommandError),

    #[error(transparent)]
    SubmitForkCommand(#[from] SubmitForkCommandError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error("Actor type forbidden")]
    ActorNotSupported(Box<dyn ActorTrait>),

    #[error(transparent)]
    Tx(#[from] sqlx::Error),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
