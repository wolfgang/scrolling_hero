use std::cell::RefCell;
use std::collections::HashMap;

use crate::combat::Combatant;
use crate::types::{DungeonLayout, Position};

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    pub guards: HashMap<(usize, usize), Combatant>,
    pub guards2: HashMap<(usize, usize), RefCell<Combatant>>,
    pub player: Combatant,
}

impl GameState {
    pub fn new(dungeon: DungeonLayout, player_position: Position) -> GameState {
        GameState {
            dungeon,
            player_position,
            guards: HashMap::new(),
            guards2: HashMap::new(),
            player: Combatant { hp: 100 },
        }
    }

    pub fn guard_at_mut(&mut self, x: usize, y: usize) -> &mut Combatant {
        self.guards.entry((x, y)).or_insert(Combatant { hp: 20 })
    }

    pub fn guard_at_ref(&mut self, x: usize, y: usize) -> &RefCell<Combatant> {
        self.guards2.entry((x, y)).or_insert(RefCell::new(Combatant { hp: 20 }))
    }
}