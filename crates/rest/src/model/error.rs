use actix_web::http::StatusCode;
use serde::Serialize;

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
