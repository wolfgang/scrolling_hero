use std::collections::HashMap;

use crate::combat::Combatant;
use crate::dungeon_helpers::make_dungeon;
use crate::game_state::GameState;

#[test]
fn construct_raw() {
    GameState {
        dungeon: vec![],
        player_position: (0, 0),
        guards: HashMap::new(),
        player: Combatant { hp: 0 },
    };
}

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec![
        "#.@#",
        "#..#"
    ]);

    let game_state = GameState::new(dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player.hp);
    assert!(game_state.guards.is_empty());
}