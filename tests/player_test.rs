use sch::player::{Player, Player0};
use sch::player::move_predicates::NonCollidingPlayerMovePredicate;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;

#[test]
fn player2_has_initial_position() {
    let predicate = NonCollidingPlayerMovePredicate::new(MAX_X, MAX_Y);
    let player = Player::new(10, 20, &predicate);
    assert_eq!((10, 20), player.position());
}


#[test]
fn player2_move_left_until_zero_x() {
    let predicate = NonCollidingPlayerMovePredicate::new(MAX_X, MAX_Y);
    let player = Player::new(2, 0, &predicate);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}


#[test]
fn has_initial_position() {
    let player = Player0::new(10, 20, MAX_X, MAX_Y);
    assert_eq!((10, 20), player.position());
}

#[test]
fn move_left_until_zero_x() {
    let player = Player0::new(2, 0, MAX_Y, MAX_Y);
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
    let player = Player0::new(MAX_X - 2, 0, MAX_X, MAX_Y);
    player.move_right();
    assert_eq!((MAX_X - 1, 0), player.position());
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
    player.move_right();
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
}

#[test]
fn move_down_until_max_y() {
    let player = Player0::new(0, MAX_Y - 2, MAX_X, MAX_Y);
    player.move_down();
    assert_eq!((0, MAX_Y - 1), player.position());
    player.move_down();
    assert_eq!((0, MAX_Y), player.position());
    player.move_down();
    player.move_down();
    assert_eq!((0, MAX_Y), player.position());
}

#[test]
fn can_move_while_borrowed_immutably() {
    let player = Player0::new(0, 0, MAX_X, MAX_Y);
    move_player_right(&player);
    assert_eq!((1, 0), player.position());
}

fn move_player_right(player: &Player0) {
    player.move_right();
}
