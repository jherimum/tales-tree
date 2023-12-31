use ::commons::{
    id::{Id, MockIdGenerator},
    time::MockClock,
};
use cqrs::{
    command_bus::{
        bus::CommandBus,
        command::create_fragment::{CreateFragmentCommand, CreateFragmentCommandBuilder},
    },
    events::FragmentCreatedEvent,
};
use sqlx::PgPool;
use std::sync::Arc;
use storage::{
    model::{event::DbEvent, user::UserBuilder},
    query::{event::QueryEvent, user::QueryUser},
};

mod commons;
mod fixtures;
mod mock;

#[sqlx::test(migrator = "storage::MIGRATOR")]
async fn test_command_bus(pool: PgPool) {
    let user = UserBuilder::default()
        .id(Id::new())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();
    let cb = CommandBus::new(
        pool.clone(),
        Arc::new(MockClock::default()),
        Arc::new(MockIdGenerator::default()),
    );

    cb.execute::<CreateFragmentCommand, _, _>(
        user,
        CreateFragmentCommandBuilder::default()
            .fragment_id(Id::new())
            .content("First tale")
            .build()
            .unwrap(),
    )
    .await
    .unwrap();

    let events = DbEvent::all(&pool).await.unwrap();

    if let Some(e) = events.first() {
        let _event_data = e.event_data().into_event::<FragmentCreatedEvent>();
        //assert_eq!(*e.event_type(), EventType::FragmentCreated);
        //assert_eq!(event_data.content(), "First tale".to_string());
    } else {
        panic!("No events found")
    }
}
