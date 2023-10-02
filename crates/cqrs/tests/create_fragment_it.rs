use crate::{
    commons::create_context,
    fixtures::user::create_user,
    mock::{clock::fixed_clock, ids::fixed_id},
};
use ::commons::{id::Id, time::DateTime};
use cqrs::{
    command_bus::{bus::Command, commands::create_fragment::CreateFragmentCommandBuilder},
    events::FragmentCreatedEventBuilder,
};
use sqlx::PgPool;
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentBuilder, FragmentState, Path},
};

mod commons;
mod fixtures;
mod mock;

#[sqlx::test(migrator = "storage::MIGRATOR")]
fn test_handle_success(pool: PgPool) {
    let created_at = DateTime::now();
    let clock = fixed_clock(created_at);

    let event_id = Id::new();
    let ids = fixed_id(event_id);

    let user = create_user(&pool).await;

    let mut ctx = create_context(&pool, &user, &clock, &ids).await;

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
                .end(false)
                .build()
                .unwrap()
        );
    } else {
        panic!("ragmentCreatedEvent should be created")
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
                .end(false)
                .created_at(created_at)
                .last_modified_at(created_at)
                .build()
                .unwrap()
        );
    } else {
        panic!("a fragment should be crated")
    }
}
