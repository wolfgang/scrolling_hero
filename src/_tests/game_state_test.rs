use crate::dungeon::helpers::make_dungeon;
use crate::game::state::GameState;

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player_ref().borrow().hp);
}

#[test]
fn guard_at_ref_gets_rc_to_guard_that_can_be_modified() {
    let (dungeon, player_pos) = make_dungeon(vec![
        "#G..#",
        "#..G#"
    ]);
    let game_state = GameState::new(dungeon.clone(), player_pos);

    let guard = game_state.guard_ref_at((1, 0));
    assert_eq!(20, guard.borrow().hp);
    guard.borrow_mut().hp = 5;
    assert_eq!(5, guard.borrow().hp);

    let guard2 = game_state.guard_ref_at((3, 1));

    assert_eq!(20, guard2.borrow().hp);
}

#[test]
fn resolve_combat_player_onehits_guard() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@.#"]);
    let mut game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(game_state.dungeon[0][1], 'G');

    game_state.guard_ref_at((1, 0)).borrow_mut().hp = 1;

    game_state.resolve_combat_at((1, 0));

    assert_eq!(game_state.dungeon[0][1], '.');
}

#[test]
fn resolve_combat_player_decreases_health_of_guard() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@.#"]);
    let mut game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(game_state.dungeon[0][1], 'G');

    let guard_ref = game_state.guard_ref_at((1, 0));
    let guard_health_before = guard_ref.borrow().hp;

    game_state.resolve_combat_at((1, 0));

    assert!(guard_ref.borrow().hp < guard_health_before);
    assert_eq!(game_state.dungeon[0][1], 'G');
}