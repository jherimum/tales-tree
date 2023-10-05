use crate::{links::ResourceLink, model::error::ErrorResponse};
use actix_web::{
    body::BoxBody,
    http::{header, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use std::error::Error;

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
