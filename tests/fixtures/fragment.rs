use chrono::Utc;
use sqlx::PgPool;
use tales_tree::{
    id::Id,
    storage::{
        active::fragment::ActiveFragment,
        model::fragment::{Fragment, FragmentBuilder, FragmentState, Path},
        model::user::User,
    },
};

pub async fn create_draft(pool: &PgPool, user: &User, content: &str) -> Fragment {
    FragmentBuilder::default()
        .id(Id::new())
        .content(String::from(content))
        .state(FragmentState::Draft)
        .parent_id(None)
        .path(Path::default())
        .author_id(*user.id())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}

pub async fn create_published(pool: &PgPool, user: &User, content: &str) -> Fragment {
    FragmentBuilder::default()
        .id(Id::new())
        .content(String::from(content))
        .state(FragmentState::Published)
        .parent_id(None)
        .path(Path::default())
        .author_id(*user.id())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}
