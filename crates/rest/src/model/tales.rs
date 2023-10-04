use crate::{Rel, ResourceLink, ResourceLinks, SingleResourceBuilder};
use actix_web::web::Path;
use commons::{fragment::Content, id::Id};
use serde::{Deserialize, Serialize};
use storage::model::fragment::Fragment;

pub type TalePath = Path<Id>;

#[derive(Deserialize, Debug)]
pub struct CreateTaleRequest {
    pub content: String,
}

impl CreateTaleRequest {
    pub fn content(&self) -> Content {
        Content::from(self.content.clone())
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateTaleRequest {
    content: Option<String>,
    end: Option<bool>,
}

impl UpdateTaleRequest {
    pub fn content(&self) -> Option<Content> {
        self.content.as_ref().map(|c| Content::from(c.clone()))
    }

    pub fn end(&self) -> Option<bool> {
        self.end
    }
}

pub struct SingleTaleResourceBuilder<'f>(&'f Fragment);

impl Into<SingleResourceBuilder<TaleResource>> for SingleTaleResourceBuilder<'_> {
    fn into(self) -> SingleResourceBuilder<TaleResource> {
        SingleResourceBuilder {
            data: Some(self.0.into()),
            links: ResourceLinks::default().add(Rel::Self_, ResourceLink::Tale(*self.0.id())),
        }
    }
}

impl From<&Fragment> for TaleResource {
    fn from(value: &Fragment) -> Self {
        TaleResource { id: *value.id() }
    }
}

#[derive(Clone, Serialize)]
pub struct TaleResource {
    pub id: Id,
}
