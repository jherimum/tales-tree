use commons::review::Comment;
use storage::model::review::ReviewAction;

#[derive(Debug, serde::Deserialize)]
pub struct CreateReviewRequest {
    action: String,
}

impl CreateReviewRequest {
    pub fn action(&self) -> ReviewAction {
        todo!()
    }

    pub fn comment(&self) -> Option<Comment> {
        todo!()
    }
}
