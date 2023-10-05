use crate::links::SingleIdPath;
use actix_web::web::Path;
use commons::fragment::Content;
use serde::Deserialize;

pub type FragmentPath = Path<SingleIdPath>;

#[derive(Deserialize, Debug)]
pub struct CreateFragmentRequest {
    pub content: String,
}

impl CreateFragmentRequest {
    pub fn content(&self) -> Content {
        Content::from(self.content.clone())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateFragmentRequest {
    content: Option<String>,
    end: Option<bool>,
}

impl UpdateFragmentRequest {
    pub fn content(&self) -> Option<Content> {
        self.content.as_ref().map(|c| Content::from(c.clone()))
    }

    pub fn end(&self) -> Option<bool> {
        self.end
    }
}
