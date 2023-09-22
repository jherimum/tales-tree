use sqlx::PgPool;
use tales_tree::{
    id::Id,
    storage::user::{User, UserBuilder},
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
