use super::{bus::Ctx, error::CommandBusError};
use crate::events::Event;
use commons::{actor::ActorTrait, commands::CommandType};
use std::fmt::Debug;

pub mod create_fragment;
pub mod delete_fragment;
pub mod dislike_fragment;
pub mod follow_user;
pub mod fork_fragment;
pub mod like_fragment;
pub mod publish_fragment;
pub mod review_fork;
pub mod submit_fork;
pub mod unfollow_user;
pub mod update_fragment;

#[async_trait::async_trait]
pub trait Command: Send + Sync + Debug {
    type Event: Event;

    fn command_type(&self) -> CommandType;

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError>;

    fn supports<A>(&self, actor: &A) -> bool
    where
        A: ActorTrait;
}
