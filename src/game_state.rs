use std::collections::HashMap;

use crate::combat::Combatant;
use crate::types::{DungeonLayout, Position};

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    pub guards: HashMap<(usize, usize), Combatant>,
    pub player: Combatant,
}
