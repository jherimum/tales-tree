use commons::{id::Id, time::DateTime};
use sqlx::PgPool;
use storage::{
    active::{fragment::ActiveFragment, user::ActiveUser},
    model::{
        fragment::{Fragment, FragmentBuilder, FragmentState, Path},
        user::{User, UserBuilder},
    },
};

async fn create_user(pool: &PgPool) -> User {
    UserBuilder::default()
        .id(Id::new())
        .build()
        .unwrap()
        .save(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn save(pool: PgPool) {
    let user = create_user(&pool).await;

    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .end(false)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let persisted_frag = Fragment::find(&pool, frag.id()).await.unwrap().unwrap();

    assert_eq!(persisted_frag, frag);
}

#[sqlx::test]
async fn find(pool: PgPool) {
    let user = create_user(&pool).await;
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .end(false)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(Fragment::find(&pool, &Id::new()).await.unwrap(), None);
    assert_eq!(Fragment::find(&pool, frag.id()).await.unwrap(), Some(frag));
}

#[sqlx::test]
async fn get_parent(pool: PgPool) {
    let user = create_user(&pool).await;
    let parent = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .end(false)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let user = create_user(&pool).await;
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .path(Path::default())
        .end(false)
        .parent_id(Some(*parent.id()))
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(parent.get_parent(&pool).await.unwrap(), None);
    assert_eq!(frag.get_parent(&pool).await.unwrap().unwrap(), parent);
}

#[sqlx::test]
fn get_children(pool: PgPool) {
    let user = create_user(&pool).await;

    let parent = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .path(Path::default())
        .end(false)
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let user = create_user(&pool).await;
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .path(Path::default())
        .end(false)
        .parent_id(Some(*parent.id()))
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert!(frag.children(&pool).await.unwrap().is_empty());

    assert_eq!(
        parent
            .children(&pool)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned(),
        frag
    );
}

#[sqlx::test]
fn update(pool: PgPool) {
    let user = create_user(&pool).await;
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(*user.id())
        .created_at(DateTime::now())
        .last_modified_at(DateTime::now())
        .state(FragmentState::Draft)
        .path(Path::default())
        .end(false)
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Fragment::find(&pool, frag.id()).await.unwrap(),
        Some(frag.clone())
    );

    let update_frag = frag
        .clone()
        .set_content("value2")
        .update(&pool)
        .await
        .unwrap();

    assert_ne!(
        Fragment::find(&pool, frag.id()).await.unwrap(),
        Some(frag.clone())
    );

    assert_eq!(
        Some(update_frag),
        Fragment::find(&pool, frag.id()).await.unwrap()
    );
}
