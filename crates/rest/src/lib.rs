use actix_web::{
    body::BoxBody,
    error::UrlGenerationError,
    http::{header, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use commons::id::{Id, IdGenerator};
use cqrs::command_bus::bus::CommandBus;
use routes::reviews::ReviewsRouter;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, sync::Arc};
use url::Url;

pub mod extractors;
pub mod model;
pub mod routes;

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
            ResourceLink::Fragment(id) => req.url_for(
                crate::routes::fragments::FragmentsRouter::SINGLE_RESOURCE_NAME,
                [id.to_string()],
            ),
            ResourceLink::Review(frag_id, review_id) => req.url_for(
                ReviewsRouter::SINGLE_RESOURCE_NAME,
                [frag_id.to_string(), review_id.to_string()],
            ),
        }
    }
}

#[derive(Clone)]
pub struct AppState<C: CommandBus> {
    command_bus: Arc<C>,
    ids: Arc<dyn IdGenerator>,
}

pub enum ApiResponse<D>
where
    D: Serialize,
{
    Created(Option<Box<dyn ResourceBuilder<D>>>, Option<ResourceLink>),
    Ok(Option<Box<dyn ResourceBuilder<D>>>),
    InternalServerError(Box<dyn Error>),
    BadRequest,
    Forbidden,
    Unauthorized,
    NotFound(&'static str),
}

pub trait ResourceBuilder<D> {
    fn build(&self, req: &HttpRequest) -> Result<D, anyhow::Error>;
}

impl<D> Responder for ApiResponse<D>
where
    D: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            ApiResponse::Created(body, link) => {
                let mut resp = HttpResponse::Created();

                if let Some(link) = link {
                    resp.append_header((header::LOCATION, link.as_url(req).unwrap().to_string()));
                }

                if let Some(body) = body {
                    return resp.json(body.build(req).unwrap());
                }

                resp.finish()
            }
            ApiResponse::InternalServerError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                ))
            }
            ApiResponse::Ok(r) => match r {
                Some(e) => HttpResponse::Ok().json(e.build(req).unwrap()),
                None => HttpResponse::Ok().finish(),
            },
            ApiResponse::BadRequest => HttpResponse::BadRequest().finish(),
            ApiResponse::Forbidden => HttpResponse::Forbidden().finish(),
            ApiResponse::Unauthorized => HttpResponse::Unauthorized().finish(),
            ApiResponse::NotFound(_) => HttpResponse::NotFound().finish(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: status.as_u16(),
            message,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
pub enum Rel {
    Self_,
    Named(&'static str),
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
