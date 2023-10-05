use crate::SingleIdPath;
use actix_web::web::Path;

pub type UserPath = Path<SingleIdPath>;

pub struct UsersRouter;

impl UsersRouter {
    pub const COLLECTION_RESOURCE_NAME: &'static str = "users";
    pub const SINGLE_RESOURCE_NAME: &'static str = "user";
}
