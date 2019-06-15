use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::combat::Combatant;
use crate::types::{CombatantRef, DungeonLayout, Position};

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    pub guards: HashMap<(usize, usize), CombatantRef>,
    pub player: Combatant,
}

impl GameState {
    pub fn new(dungeon: DungeonLayout, player_position: Position) -> GameState {
        let mut guards = HashMap::new();

        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    guards.insert((x, y), Rc::new(RefCell::new(Combatant { hp: 20 })));
                }
            }
        }

        GameState {
            dungeon,
            player_position,
            guards,
            player: Combatant { hp: 100 },
        }
    }

    pub fn guard_at_ref(&self, x: usize, y: usize) -> CombatantRef {
        self.guards.get(&(x, y)).unwrap().clone()
    }
}