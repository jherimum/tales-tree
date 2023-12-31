use commons::{id::Id, time::DateTime};
use sqlx::PgPool;
use storage::{
    model::{
        fragment::{Fragment, FragmentBuilder, FragmentState, Path},
        user::User,
    },
    query::fragment::QueryFragment,
};

pub async fn create_draft(pool: &PgPool, user: &User, content: &str, end: bool) -> Fragment {
    FragmentBuilder::default()
        .id(Id::new())
        .content(String::from(content))
        .state(FragmentState::Draft)
        .parent_id(None)
        .path(Path::default())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .end(end)
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}

pub async fn create_published(pool: &PgPool, user: &User, content: &str, end: bool) -> Fragment {
    FragmentBuilder::default()
        .id(Id::new())
        .content(String::from(content))
        .state(FragmentState::Published)
        .parent_id(None)
        .path(Path::default())
        .end(end)
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}
