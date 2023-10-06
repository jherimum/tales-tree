use crate::response::ApiResponse;

pub struct HealthRouter;

impl HealthRouter {
    pub const HEALTH_RESOURCE_NAME: &str = "health";

    pub async fn get() -> ApiResponse<()> {
        ApiResponse::Ok(None)
    }
}
