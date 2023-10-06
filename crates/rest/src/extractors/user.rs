use actix_web::FromRequest;
use std::future::Ready;
use storage::model::user::User;

pub struct UserExtractor(pub User);

impl FromRequest for UserExtractor {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        todo!()
    }
}
