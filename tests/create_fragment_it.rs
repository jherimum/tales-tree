use crate::commons::create_user;
use sqlx::PgPool;
use tales_tree::{
    actor::Actor,
    commands::{create_fragment::CreateFragmentCommand, CommandHandler, CommandHandlerContext},
    id::Id,
    storage::fragment::{Fragment, FragmentState, Path},
};

mod commons;

#[sqlx::test]
fn test_handle_success(pool: PgPool) {
    let user = create_user(&pool).await;

    let command = CreateFragmentCommand {
        id: Id::new(),
        content: "Frament".to_owned(),
    };
    let mut ctx = CommandHandlerContext::new(&pool, &Actor::User(user.clone()))
        .await
        .unwrap();

    let result = command.handle(&mut ctx).await.unwrap();
    if let Some(e) = result {
        assert_eq!(e.user_id, *user.id());
        assert_eq!(e.fragment_id, command.id);
        assert_eq!(e.content, command.content);
    } else {
        panic!("a fragment should be crated")
    }

    let frag = Fragment::find(ctx.tx().as_mut(), &command.id)
        .await
        .unwrap();
    if let Some(frag) = frag {
        assert_eq!(*frag.id(), command.id);
        assert_eq!(frag.author_id(), user.id());
        assert_eq!(frag.content(), "Frament");
        assert_eq!(*frag.state(), FragmentState::Draft);
        assert_eq!(frag.parent_id().as_ref(), None);
        assert_eq!(frag.path(), &Path::empty());
    } else {
        panic!("a fragment should be crated")
    }
}
