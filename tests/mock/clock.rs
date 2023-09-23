use tales_tree::{clock::MockClock, DateTime};

pub fn fixed_clock(time: DateTime) -> MockClock {
    let mut clock = MockClock::default();
    clock.expect_now().returning(move || time.clone());
    clock
}
