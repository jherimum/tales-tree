use crate::{
    extractors::user::UserExtractor,
    model::{fragments::FragmentPath, reviews::CreateReviewRequest},
    ApiResponse, AppState, ResourceLink,
};
use actix_web::web::{Data, Json};
use cqrs::command_bus::{bus::CommandBus, command::review_fork::ReviewForkCommandBuilder};

pub struct ReviewsRouter;

impl ReviewsRouter {
    pub const COLLECTION_RESOURCE_NAME: &str = "reviews";
    pub const SINGLE_RESOURCE_NAME: &str = "review";

    pub async fn create<C: CommandBus>(
        state: Data<AppState<C>>,
        fragment_path: FragmentPath,
        Json(payload): Json<CreateReviewRequest>,
        UserExtractor(user): UserExtractor,
    ) -> ApiResponse<()> {
        let review_id = state.ids.new_id();
        let command = ReviewForkCommandBuilder::default()
            .review_id(review_id)
            .fragment_id(fragment_path.as_ref())
            .action(payload.action())
            .comment(payload.comment().clone())
            .build()
            .unwrap();

        match state.command_bus.execute(user, command).await {
            Ok(_) => ApiResponse::Created(
                None,
                Some(ResourceLink::Review(
                    fragment_path.as_ref().into(),
                    review_id,
                )),
            ),
            Err(e) => ApiResponse::InternalServerError(e.into()),
        }
    }
}
