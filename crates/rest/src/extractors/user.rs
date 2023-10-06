use actix_web::{web::Data, FromRequest};
use commons::id::Id;
use std::{future::Future, pin::Pin};
use storage::{model::user::User, query::user::QueryUser};

use crate::server::AppState;

const USER_ID_HEADER_KEY: &str = "user-id";
pub struct UserExtractor(pub User);

impl FromRequest for UserExtractor {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let state = req.app_data::<Data<AppState>>().unwrap();
            let user_id: Id = req
                .headers()
                .get(USER_ID_HEADER_KEY)
                .unwrap()
                .to_str()
                .unwrap()
                .try_into()
                .unwrap();

            let user = User::find(&state.pool, &user_id).await.unwrap().unwrap();

            Ok(UserExtractor(user))
        })
    }
}
