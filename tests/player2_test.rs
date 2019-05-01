use std::rc::Rc;

use sch::move_predicate::MovePredicate;
use sch::player::move_predicates::NonCollidingPlayerMovePredicate;
use sch::player::Player2;

mod player_factory;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;

struct TestPlayerBuilder {
    position: (u32, u32),
    bounds: (u32, u32),
}

impl TestPlayerBuilder {
    pub fn player_at(x: u32, y: u32) -> TestPlayerBuilder {
        TestPlayerBuilder { position: (x, y), bounds: (10000, 10000) }
    }

    pub fn with_bounds(mut self, max_x: u32, max_y: u32) -> TestPlayerBuilder {
        self.bounds = (max_x, max_y);
        self
    }

    pub fn build(self) -> Player2 {
        let predicate: Rc<MovePredicate> = Rc::new(NonCollidingPlayerMovePredicate::new(self.bounds.0, self.bounds.1));
        Player2::new(self.position.0, self.position.1, &predicate)
    }
}

#[test]
fn player2_has_initial_position() {
    let player = TestPlayerBuilder::player_at(10, 20).build();
//        let player = Player2::new_default(10, 20, 100, 200);
    assert_eq!((10, 20), player.position());
}


#[test]
fn move_left_until_zero_x() {
    let player = player_factory::without_bounds(2, 0);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}

#[test]
fn move_right_until_max_x() {
    let player = TestPlayerBuilder::player_at(MAX_X - 2, 0)
        .with_bounds(MAX_X, MAX_Y)
        .build();
    player.move_right();
    assert_eq!((MAX_X - 1, 0), player.position());
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
    player.move_right();
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
}
