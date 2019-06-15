use crate::dungeon_helpers::make_dungeon;
use crate::game_state::GameState;

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player.borrow().hp);
    assert!(game_state.guards.is_empty());
}

#[test]
fn guard_at_ref_gets_rc_to_guard_that_can_be_modified() {
    let (dungeon, player_pos) = make_dungeon(vec![
        "#G..#",
        "#..G#"
    ]);
    let game_state = GameState::new(dungeon.clone(), player_pos);

    let guard = game_state.guard_at_ref(1, 0);
    assert_eq!(20, guard.borrow().hp);
    guard.borrow_mut().hp = 5;
    assert_eq!(5, guard.borrow().hp);

    let guard2 = game_state.guard_at_ref(3, 1);

    assert_eq!(20, guard2.borrow().hp);
}
