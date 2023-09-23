use tales_tree::id::{Id, MockIdGenerator};

pub fn fixed_id(id: Id) -> MockIdGenerator {
    let mut ids = MockIdGenerator::default();
    ids.expect_new().returning(move || id.clone());
    ids
}
