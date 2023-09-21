use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    id::Id,
    storage::{
        fragment::{Fragment, FragmentState},
        review::{Review, ReviewAction, ReviewBuilder},
        user::User,
    },
};
use chrono::Utc;
use tap::TapFallible;

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
impl CommandHandler for ReviewForkCommand {
    type Output = Review;

    fn supports(&self, actor: &crate::actor::Actor) -> bool {
        actor.is_user()
    }

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let frag = Fragment::find(ctx.pool, &self.fragment_id)
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

        if !parent.is_author(&user) {
            return Err(ReviewForkCommandError::InvalidState(
                "only the parent author can review a fork",
            )
            .into());
        }

        let review = ReviewBuilder::default()
            .id(self.review_id)
            .fragment_id(self.fragment_id)
            .reviewer_id(*user.id())
            .comment(self.comment.clone())
            .created_at(Utc::now().naive_utc())
            .action(self.action)
            .build()
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save review: {e}"))?;

        frag.set_state(FragmentState::from(self.action))
            .set_last_modified_at(Utc::now().naive_utc())
            .update(ctx.tx().as_mut())
            .await?;

        Ok(review)
    }
}
