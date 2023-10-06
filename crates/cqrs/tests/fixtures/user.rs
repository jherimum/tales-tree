use commons::id::Id;
use sqlx::PgPool;
use storage::{
    model::user::{User, UserBuilder},
    query::user::QueryUser,
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
