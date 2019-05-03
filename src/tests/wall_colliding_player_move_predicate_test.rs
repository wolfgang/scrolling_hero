use std::rc::Rc;

use crate::move_predicate::MovePredicate;
use crate::player::move_predicates::WallCollidingPlayerMovePredicate;
use crate::player::Player;
use crate::types::Dungeon;

#[test]
fn can_not_move_right_if_wall_is_to_the_right() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0]
    ];
    let player = player_at(1, 1, dungeon);
    player.move_right();
    assert_eq!((1, 1), player.position());
}

#[test]
fn can_move_right_if_no_wall_is_to_the_right() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0]
    ];
    let player = player_at(1, 1, dungeon);
    player.move_right();
    assert_eq!((2, 1), player.position());
}

#[test]
fn can_not_move_left_if_wall_is_to_the_left() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 0, 0]
    ];
    let player = player_at(1, 1, dungeon);
    player.move_left();
    assert_eq!((1, 1), player.position());
}

#[test]
fn can_move_left_if_no_wall_is_to_the_left() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0]
    ];
    let player = player_at(1, 1, dungeon);
    player.move_left();
    assert_eq!((0, 1), player.position());
}

#[test]
fn can_not_move_down_if_wall_is_below() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0]
    ];
    let player = player_at(1, 0, dungeon);
    player.move_down();
    assert_eq!((1, 0), player.position());
}

#[test]
fn can_not_move_down_if_no_wall_is_below() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0]
    ];
    let player = player_at(1, 0, dungeon);
    player.move_down();
    assert_eq!((1, 1), player.position());
}

fn player_at(x: u32, y: u32, dungeon: Dungeon) -> Player {
    let predicate = WallCollidingPlayerMovePredicate::new(&Rc::new(dungeon));
    Player::new(x, y, &(Rc::new(predicate) as Rc<MovePredicate>))
}