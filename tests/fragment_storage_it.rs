use chrono::Utc;
use sqlx::PgPool;
use tales_tree::{
    id::Id,
    storage::{
        active::fragment::ActiveFragment,
        fragment::{Fragment, FragmentBuilder, FragmentState, Path},
    },
};

#[sqlx::test]
async fn save(pool: PgPool) {
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
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
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert!(Fragment::find(&pool, &Id::new()).await.unwrap().is_none());
    assert!(Fragment::find(&pool, frag.id()).await.unwrap().is_some());
}

#[sqlx::test]
async fn get_parent(pool: PgPool) {
    let parent = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
        .parent_id(Some(*parent.id()))
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert!(parent.get_parent(&pool).await.unwrap().is_none());
    assert_eq!(frag.get_parent(&pool).await.unwrap().unwrap(), parent);
}

#[sqlx::test]
fn get_children(pool: PgPool) {
    let parent = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
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
    let frag = FragmentBuilder::default()
        .id(Id::new())
        .content("value".to_string())
        .author_id(Id::new())
        .created_at(Utc::now().naive_utc())
        .last_modified_at(Utc::now().naive_utc())
        .state(FragmentState::Draft)
        .path(Path::default())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        frag,
        Fragment::find(&pool, frag.id()).await.unwrap().unwrap()
    );

    let update_frag = frag
        .clone()
        .set_content("value2")
        .update(&pool)
        .await
        .unwrap();

    assert_ne!(
        frag,
        Fragment::find(&pool, frag.id()).await.unwrap().unwrap()
    );

    assert_eq!(
        update_frag,
        Fragment::find(&pool, frag.id()).await.unwrap().unwrap()
    );
}
