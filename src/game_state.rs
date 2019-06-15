use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::combat::Combatant;
use crate::types::{DungeonLayout, Position};

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    pub guards: HashMap<(usize, usize), Rc<RefCell<Combatant>>>,
    pub player: Combatant,
}

impl GameState {
    pub fn new(dungeon: DungeonLayout, player_position: Position) -> GameState {
        let mut guards2 = HashMap::new();

        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    guards2.insert((x, y), Rc::new(RefCell::new(Combatant { hp: 20 })));
                }
            }
        }

        GameState {
            dungeon,
            player_position,
            guards: guards2,
            player: Combatant { hp: 100 },
        }
    }

    pub fn guard_at_ref(&self, x: usize, y: usize) -> Rc<RefCell<Combatant>> {
        self.guards.get(&(x, y)).unwrap().clone()
    }
}