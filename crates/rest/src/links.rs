use crate::routes::{fragments::FragmentsRouter, reviews::ReviewsRouter};
use actix_web::{error::UrlGenerationError, HttpRequest};
use commons::id::Id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Rel {
    Self_,
    Named(&'static str),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SingleIdPath(Id);

impl From<SingleIdPath> for Id {
    fn from(value: SingleIdPath) -> Self {
        value.0
    }
}

impl From<&SingleIdPath> for Id {
    fn from(value: &SingleIdPath) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub enum ResourceLink {
    Fragment(Id),
    Review(Id, Id),
}

impl ResourceLink {
    pub fn as_url(&self, req: &actix_web::HttpRequest) -> Result<Url, UrlGenerationError> {
        match self {
            ResourceLink::Fragment(id) => {
                req.url_for(FragmentsRouter::SINGLE_RESOURCE_NAME, [id.to_string()])
            }
            ResourceLink::Review(frag_id, review_id) => req.url_for(
                ReviewsRouter::SINGLE_RESOURCE_NAME,
                [frag_id.to_string(), review_id.to_string()],
            ),
        }
    }
}

#[derive(Default)]
pub struct ResourceLinks(HashMap<Rel, ResourceLink>);

impl ResourceLinks {
    pub fn add(self, rel: Rel, link: ResourceLink) -> Self {
        let mut map = self.0;
        map.insert(rel, link);
        Self(map)
    }

    pub fn build(self, req: &HttpRequest) -> HashMap<Rel, Url> {
        self.0
            .into_iter()
            .map(|(rel, l)| (rel, l.as_url(req).unwrap()))
            .collect()
    }
}
