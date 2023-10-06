use crate::{
    links::ResourceLink,
    model::error::{self, ErrorResponse},
};
use actix_web::{
    body::BoxBody,
    http::{header, StatusCode},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use serde::Serialize;
use std::error::Error;

pub enum ApiResponse<D>
where
    D: Serialize,
{
    Created(Option<Box<dyn ResourceBuilder<D>>>, Option<ResourceLink>),
    Ok(Option<Box<dyn ResourceBuilder<D>>>),
    Error(ApiError),
}

#[derive(Debug, thiserror::Error)]
#[error("API error")]
pub enum ApiError {
    InternalServerError(Box<dyn Error>),
    BadRequest,
    Forbidden,
    Unauthorized,
    NotFound(&'static str),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).finish()
    }
}

impl<D> Into<ApiResponse<D>> for ApiError
where
    D: Serialize,
{
    fn into(self) -> ApiResponse<D> {
        ApiResponse::Error(self)
    }
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
            ApiResponse::Ok(r) => match r {
                Some(e) => HttpResponse::Ok().json(e.build(req).unwrap()),
                None => HttpResponse::Ok().finish(),
            },
            ApiResponse::Error(e) => match e {
                ApiError::InternalServerError(_) => {
                    HttpResponse::InternalServerError().json(ErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    ))
                }
                ApiError::BadRequest => HttpResponse::BadRequest().finish(),
                ApiError::Forbidden => HttpResponse::Forbidden().finish(),
                ApiError::Unauthorized => HttpResponse::Unauthorized().finish(),
                ApiError::NotFound(_) => HttpResponse::NotFound().finish(),
            },
        }
    }
}
