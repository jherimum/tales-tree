use commons::id::Id;
use sqlx::PgPool;
use storage::{
    active::user::ActiveUser,
    model::user::{User, UserBuilder},
};

pub async fn create_user(pool: &PgPool) -> User {
    UserBuilder::default()
        .id(Id::new())
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}
