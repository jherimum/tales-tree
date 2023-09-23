mod commons;
mod fixtures;
mod mock;

use crate::{
    commons::create_context,
    fixtures::{fragment::create_draft, user::create_user},
    mock::{clock::fixed_clock, ids::fixed_id},
};
use chrono::Utc;
use sqlx::PgPool;
use tales_tree::{
    commands::{update_fragment::UpdateFragmentCommandBuilder, CommandHandler},
    id::Id,
    storage::fragment::Fragment,
};

#[sqlx::test]
fn test_success_update(pool: PgPool) {
    let user = create_user(&pool).await;
    let draf = create_draft(&pool, &user, "content").await;

    let command = UpdateFragmentCommandBuilder::default()
        .fragment_id(*draf.id())
        .content("New Content".to_owned())
        .build()
        .unwrap();

    let now = Utc::now().naive_utc();
    let mut ctx = create_context(&pool, user, fixed_clock(now), fixed_id(Id::new())).await;

    let event = command.handle(&mut ctx).await.unwrap().unwrap();

    let fragment = Fragment::find(ctx.tx().as_mut(), draf.id())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(fragment.content(), "New Content");
    assert_eq!(*fragment.last_modified_at(), now);
}
