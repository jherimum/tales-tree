use chrono::Utc;
use sqlx::{Connection, PgConnection, PgPool};
use tales_tree::{
    id::Id,
    storage::fragment::{Fragment, FragmentBuilder, FragmentState, Path},
};

#[tokio::test]
async fn test() {
    let mut conn = PgConnection::connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .unwrap();

    let mut path = Path::default();
    path.append(Id::new());
    path.append(Id::new());
    path.append(Id::new());
    path.append(Id::new());
    FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(path)
        .build()
        .unwrap()
        .save(conn.as_mut())
        .await
        .unwrap();
}
