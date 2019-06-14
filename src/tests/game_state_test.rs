use crate::dungeon_helpers::make_dungeon;
use crate::game_state::GameState;

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player.hp);
    assert!(game_state.guards.is_empty());
}

#[test]
fn guard_at_mut_inserts_if_not_present_and_returns_mutable_reference() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let mut game_state = GameState::new(dungeon.clone(), player_pos);

    {
        let mut guard = game_state.guard_at_mut(1, 2);
        assert_eq!(20, guard.hp);
        guard.hp = 5;
        assert_eq!(5, guard.hp);
    }

    let guard2 = game_state.guard_at_mut(1, 2);
    assert_eq!(5, guard2.hp);
}