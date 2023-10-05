use std::collections::HashMap;

use actix_web::HttpRequest;
use serde::Serialize;
use url::Url;

use crate::links::{Rel, ResourceLinks};

pub struct SingleResourceBuilder<D> {
    data: Option<D>,
    links: ResourceLinks,
}

impl<D> SingleResourceBuilder<D> {
    pub fn build(self, req: &HttpRequest) -> Result<SingleResource<D>, anyhow::Error> {
        Ok(SingleResource {
            data: self.data,
            links: self.links.build(req),
        })
    }
}

pub struct CollectionResourceBuilder<D> {
    data: Vec<SingleResourceBuilder<D>>,
    links: ResourceLinks,
}

impl<D> CollectionResourceBuilder<D> {
    fn build(self, req: &HttpRequest) -> Result<CollectionResource<D>, anyhow::Error> {
        Ok(CollectionResource {
            data: self
                .data
                .into_iter()
                .map(|b| b.build(req).unwrap())
                .collect::<Vec<_>>(),
            links: self.links.build(req),
        })
    }
}

#[derive(Serialize)]
pub struct SingleResource<D> {
    #[serde(flatten)]
    data: Option<D>,
    links: HashMap<Rel, Url>,
}

#[derive(Serialize)]
pub struct CollectionResource<D> {
    data: Vec<SingleResource<D>>,
    links: HashMap<Rel, Url>,
}
