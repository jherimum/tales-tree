use actix_web::web::Path;

use crate::SingleIdPath;

pub type UserPath = Path<SingleIdPath>;
