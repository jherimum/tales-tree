use crate::command_bus::bus::Ctx;
use crate::command_bus::{bus::Command, error::CommandBusError};
use crate::events::FragmentForkReviewedEvent;
use commons::{commands::CommandType, id::Id};
use storage::{
    active::{fragment::ActiveFragment, review::ActiveReview},
    model::{
        fragment::{Fragment, FragmentState},
        review::{Review, ReviewAction, ReviewBuilder},
    },
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct ReviewForkCommand {
    pub review_id: Id,
    pub fragment_id: Id,
    pub action: ReviewAction,
    pub comment: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ReviewForkCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("{0}")]
    InvalidState(&'static str),
}

#[async_trait::async_trait]
impl Command for ReviewForkCommand {
    type Event = FragmentForkReviewedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::ReviewFork
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        //actor.is_user()
        todo!()
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();
        let frag = Fragment::find(ctx.pool(), &self.fragment_id)
            .await?
            .ok_or(ReviewForkCommandError::FragmentNotFound(self.fragment_id))?;

        if !frag.is_fork() {
            return Err(ReviewForkCommandError::InvalidState(
                "fragment should be in fork state to be reviewed",
            )
            .into());
        }

        if !frag.is_waiting_review() {
            return Err(ReviewForkCommandError::InvalidState(
                "fragment should be in waiting review state to be reviewed",
            )
            .into());
        }

        let parent = frag.get_parent(ctx.pool()).await?.unwrap();

        if !parent.is_author(user) {
            return Err(ReviewForkCommandError::InvalidState(
                "only the parent author can review a fork",
            )
            .into());
        }

        let review = ReviewBuilder::default()
            .id(self.review_id)
            .fragment_id(self.fragment_id)
            .reviewer_id(user)
            .comment(self.comment.clone())
            .created_at(ctx.clock().now())
            .action(self.action)
            .build()
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save review: {e}"))?;

        frag.set_state(FragmentState::from(self.action))
            .set_last_modified_at(ctx.clock().now())
            .update(ctx.tx().as_mut())
            .await?;

        Ok(Some(review.into()))
    }
}

impl From<Review> for FragmentForkReviewedEvent {
    fn from(value: Review) -> Self {
        FragmentForkReviewedEvent {
            fragment_id: *value.fragment_id(),
            action: *value.action(),
            comment: value.comment().clone(),
            timestamp: *value.created_at(),
        }
    }
}
