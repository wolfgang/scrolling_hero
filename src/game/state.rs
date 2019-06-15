use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::{CombatantRef, DungeonLayout, Position};

use super::combat::Combatant;

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    guards: HashMap<(usize, usize), CombatantRef>,
    player: CombatantRef,
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
            player: Rc::new(RefCell::new(Combatant { hp: 100 }))
        }
    }

    pub fn guard_ref_at(&self, pos: (usize, usize)) -> CombatantRef {
        self.guards.get(&pos).unwrap().clone()
    }

    pub fn player_ref(&self) -> CombatantRef {
        self.player.clone()
    }
}