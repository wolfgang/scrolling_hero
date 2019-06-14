use std::collections::HashMap;

use crate::combat::Combatant;
use crate::game_state::GameState;

#[test]
fn construct() {
    let game_state = GameState {
        dungeon: vec![],
        player_position: (0, 0),
        guards: HashMap::new(),
        player: Combatant { hp: 0 },
    };
}