use crate::{commons::create_context, fixtures::user::create_user};
use chrono::Utc;
use sqlx::PgPool;
use tales_tree::{
    clock::MockClock,
    commands::{create_fragment::CreateFragmentCommandBuilder, CommandHandler},
    events::FragmentCreatedEventBuilder,
    id::{Id, MockIdGenerator},
    storage::fragment::{Fragment, FragmentBuilder, FragmentState, Path},
    DateTime,
};

mod commons;
mod fixtures;

#[sqlx::test]
fn test_handle_success(pool: PgPool) {
    let created_at = Utc::now().naive_utc();
    let clock = prepare_clock(created_at);

    let event_id = Id::new();
    let ids = prepare_ids(event_id);

    let user = create_user(&pool).await;

    let mut ctx = create_context(&pool, user.clone(), clock, ids).await;

    let command = CreateFragmentCommandBuilder::default()
        .fragment_id(Id::new())
        .content("Fragment".to_owned())
        .build()
        .unwrap();

    let result = command.handle(&mut ctx).await.unwrap();
    if let Some(e) = result {
        assert_eq!(
            e,
            FragmentCreatedEventBuilder::default()
                .fragment_id(*command.fragment_id())
                .content(command.content().clone())
                .user_id(*user.id())
                .timestamp(created_at)
                .build()
                .unwrap()
        );
    } else {
        panic!("a fragment should be crated")
    }

    let frag = Fragment::find(ctx.tx().as_mut(), command.fragment_id())
        .await
        .unwrap();
    if let Some(frag) = frag {
        assert_eq!(
            frag,
            FragmentBuilder::default()
                .id(*command.fragment_id())
                .author_id(user.id().clone())
                .content("Fragment".to_owned())
                .state(FragmentState::Draft)
                .parent_id(None)
                .path(Path::empty())
                .created_at(created_at)
                .last_modified_at(created_at)
                .build()
                .unwrap()
        );
    } else {
        panic!("a fragment should be crated")
    }
}

fn prepare_clock(time: DateTime) -> MockClock {
    let mut clock = MockClock::default();
    clock.expect_now().returning(move || time.clone());
    clock
}

fn prepare_ids(id: Id) -> MockIdGenerator {
    let mut ids = MockIdGenerator::default();
    ids.expect_new().returning(move || id.clone());
    ids
}
