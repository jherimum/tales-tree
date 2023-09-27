use crate::command_bus::{bus::Command, bus::CommandHandlerContext, error::CommandBusError};
use crate::events::UserUnfollowedEvent;
use commons::{
    actor::ActorTrait,
    commands::CommandType,
    id::{Id, IdGenerator},
    time::Clock,
};
use storage::{active::follow::ActiveFollow, model::follow::Follow};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct UnfollowUserCommand {
    followee_user_id: Id,
}

#[async_trait::async_trait]
impl Command for UnfollowUserCommand {
    type Event = UserUnfollowedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::UnfollowUser
    }

    async fn handle<A, CL, I>(
        &self,
        ctx: &mut CommandHandlerContext<A, CL, I>,
    ) -> Result<Option<Self::Event>, CommandBusError>
    where
        A: ActorTrait,
        CL: Clock,
        I: IdGenerator,
    {
        let user = ctx.actor().id().unwrap();
        let actual_follow = Follow::find(ctx.pool(), &user, &self.followee_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        match actual_follow {
            Some(f) => match f.delete(ctx.tx().as_mut()).await? {
                true => Ok(Some(UserUnfollowedEvent {
                    follower_id: user,
                    followee_id: self.followee_user_id,
                    timestamp: *f.created_at(),
                })),
                false => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        //actor.is_user()
        todo!()
    }
}
