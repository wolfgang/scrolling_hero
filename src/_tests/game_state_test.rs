use crate::dungeon::helpers::make_dungeon;
use crate::game::state::GameState;

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let game_state = GameState::new(dungeon.clone(), player_pos, 20);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player_ref().borrow().hp);
}
