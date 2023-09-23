mod commons;
mod fixtures;
mod mock;

use crate::{
    commons::create_context,
    fixtures::{
        fragment::{create_draft, create_published},
        user::create_user,
    },
    mock::{clock::fixed_clock, ids::fixed_id},
};
use chrono::Utc;
use sqlx::PgPool;
use tales_tree::{
    clock::MockClock,
    commands::{
        update_fragment::{UpdateFragmentCommandBuilder, UpdateFragmentCommandError},
        CommandBusError, CommandHandler,
    },
    events::FragmentUpdatedEventBuilder,
    id::{Id, MockIdGenerator},
    storage::{
        active::fragment::ActiveFragment,
        fragment::{Fragment, FragmentBuilder},
    },
};

#[sqlx::test]
fn test_success_draft_update(pool: PgPool) {
    const OLD_CONTENT: &str = "content";
    const NEW_CONTENT: &str = "new content";

    let user = create_user(&pool).await;
    let draft = create_draft(&pool, &user, OLD_CONTENT).await;

    let command = UpdateFragmentCommandBuilder::default()
        .fragment_id(*draft.id())
        .content(NEW_CONTENT)
        .build()
        .unwrap();

    let now = Utc::now().naive_utc();
    let mut ctx = create_context(&pool, &user, fixed_clock(now), fixed_id(Id::new())).await;

    let result = command.handle(&mut ctx).await.unwrap();

    if let Some(e) = result {
        assert_eq!(
            e,
            FragmentUpdatedEventBuilder::default()
                .fragment_id(*draft.id())
                .content(NEW_CONTENT)
                .user_id(*user.id())
                .timestamp(now.clone())
                .build()
                .unwrap()
        )
    } else {
        panic!("Expected Some(FragmentUpdatedEvent) but got None");
    }

    let fragment = Fragment::find(ctx.tx().as_mut(), draft.id())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        fragment,
        FragmentBuilder::default()
            .id(*draft.id())
            .author_id(*user.id())
            .content(NEW_CONTENT)
            .state(draft.state().clone())
            .parent_id(draft.parent_id().clone())
            .created_at(*draft.created_at())
            .last_modified_at(now)
            .build()
            .unwrap()
    );
}

#[sqlx::test]
fn test_actor_not_author(pool: PgPool) {
    let author = create_user(&pool).await;
    let draft = create_draft(&pool, &author, "content").await;
    let other_user = create_user(&pool).await;

    let command = UpdateFragmentCommandBuilder::default()
        .fragment_id(*draft.id())
        .content("new content")
        .build()
        .unwrap();

    let mut ctx = create_context(
        &pool,
        &other_user,
        MockClock::default(),
        MockIdGenerator::default(),
    )
    .await;

    match command.handle(&mut ctx).await {
        Ok(_) => panic!("Expected Err(CommandBusError) but got Ok(_)"),
        Err(CommandBusError::UpdateFragmentCommand(e)) => assert_eq!(
            e,
            UpdateFragmentCommandError::UserWithoutPermission(*other_user.id())
        ),
        Err(_) => panic!("Not expected error"),
    }
}

#[sqlx::test]
fn test_non_editable_fragment(pool: PgPool) {
    let author = create_user(&pool).await;
    let published = create_published(&pool, &author, "content").await;

    let command = UpdateFragmentCommandBuilder::default()
        .fragment_id(*published.id())
        .content("new content")
        .build()
        .unwrap();

    let mut ctx = create_context(
        &pool,
        &author,
        MockClock::default(),
        MockIdGenerator::default(),
    )
    .await;

    match command.handle(&mut ctx).await {
        Ok(_) => panic!("Expected Err(CommandBusError) but got Ok(_)"),
        Err(CommandBusError::UpdateFragmentCommand(e)) => assert_eq!(
            e,
            UpdateFragmentCommandError::NonEditableFragment(*published.id())
        ),
        Err(_) => panic!("Not expected error"),
    }
}

#[sqlx::test]
fn test_fragment_not_found(pool: PgPool) {
    let user = create_user(&pool).await;

    let command = UpdateFragmentCommandBuilder::default()
        .fragment_id(Id::new())
        .content("new content")
        .build()
        .unwrap();

    let mut ctx = create_context(
        &pool,
        &user,
        MockClock::default(),
        MockIdGenerator::default(),
    )
    .await;

    match command.handle(&mut ctx).await {
        Ok(_) => panic!("Expected Err(CommandBusError) but got Ok(_)"),
        Err(CommandBusError::UpdateFragmentCommand(e)) => assert_eq!(
            e,
            UpdateFragmentCommandError::FragmentNotFound(*command.fragment_id())
        ),
        Err(_) => panic!("Not expected error"),
    }
}
