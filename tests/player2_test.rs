use std::rc::Rc;

use sch::move_predicate::MovePredicate;
use sch::player::Player2;
use sch::player::move_predicates::NonCollidingPlayerMovePredicate;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;

#[test]
fn player2_has_initial_position() {
    let player = Player2::new_default(10, 20, 100, 200);
    assert_eq!((10, 20), player.position());
}


#[test]
fn move_left_until_zero_x() {
    let player = Player2::new_default(2, 0, MAX_X, MAX_Y);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}

#[test]
fn move_left_until_zero_x_can_pass_predicate() {
    let predicate: Rc<MovePredicate> = Rc::new(NonCollidingPlayerMovePredicate::new(MAX_X, MAX_Y));
    let player = Player2::new(2, 0, &predicate);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}
