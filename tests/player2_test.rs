use speculate::speculate;

use sch::player::move_predicates::NonCollidingPlayerMovePredicate;
use sch::player::Player2;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;

speculate! {

before {
    let move_predicate = NonCollidingPlayerMovePredicate::new(MAX_X, MAX_Y);
}

it "player2_has_initial_position" {
    let player = Player2::new(10, 20, &move_predicate);
    assert_eq!((10, 20), player.position());
}


it "player2_move_left_until_zero_x" {
    let player = Player2::new(2, 0, &move_predicate);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}


it "move_right_until_max_x" {
    let player = Player2::new(MAX_X - 2, 0, &move_predicate);
    player.move_right();
    assert_eq!((MAX_X - 1, 0), player.position());
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
    player.move_right();
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
}

it "move_down_until_max_y" {
    let player = Player2::new(0, MAX_Y - 2, &move_predicate);
    player.move_down();
    assert_eq!((0, MAX_Y - 1), player.position());
    player.move_down();
    assert_eq!((0, MAX_Y), player.position());
    player.move_down();
    player.move_down();
    assert_eq!((0, MAX_Y), player.position());
}


it "can_move_while_borrowed_immutably" {
    let player = Player2::new(0, 0, &move_predicate);
    move_player_right(&player);
    assert_eq!((1, 0), player.position());
}

fn move_player_right(player: &Player2) {
    player.move_right();
}

}