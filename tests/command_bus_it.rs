use std::sync::Arc;

use sqlx::PgPool;
use tales_tree::{
    actor::Actor,
    clock::MockClock,
    commands::{
        create_fragment::{
            CreateFragmentCommand, CreateFragmentCommandBuilder, CreateFragmentCommandError,
        },
        CommandBus,
    },
    events::{EventType, FragmentCreatedEvent},
    id::{Id, MockIdGenerator},
    storage::{event::DbEvent, user::UserBuilder},
};

mod commons;
mod fixtures;

#[sqlx::test]
async fn test_command_bus(pool: PgPool) {
    // let user = UserBuilder::default()
    //     .id(Id::new())
    //     .build()
    //     .unwrap()
    //     .save(&pool)
    //     .await
    //     .unwrap();
    // let cb = CommandBus::new(
    //     pool.clone(),
    //     Arc::new(MockClock::default()),
    //     Arc::new(MockIdGenerator::default()),
    // );

    // cb.execute::<CreateFragmentCommand, CreateFragmentCommandError>(
    //     &Actor::User(user.clone()),
    //     CreateFragmentCommandBuilder::default()
    //         .fragment_id(Id::new())
    //         .content("First tale".to_string())
    //         .build()
    //         .unwrap(),
    // )
    // .await
    // .unwrap();

    // let events = DbEvent::all(&pool).await.unwrap();

    // if let Some(e) = events.first() {
    //     let event_data = e.event_data().into_event::<FragmentCreatedEvent>();
    //     assert_eq!(*e.event_type(), EventType::FragmentCreated);
    //     assert_eq!(event_data.content, "First tale".to_string());
    // } else {
    //     panic!("No events found")
    // }
}
