use std::collections::HashMap;

use crate::types::{CombatantRef, DungeonLayout, Position};

use super::combat::Combatant;

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    guards: HashMap<Position, CombatantRef>,
    player: CombatantRef,
}

impl GameState {
    pub fn new(dungeon: DungeonLayout, player_position: Position) -> GameState {
        let mut guards = HashMap::new();

        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    guards.insert((x as u32, y as u32), Combatant::ref_with_hp(20));
                }
            }
        }

        GameState {
            dungeon,
            player_position,
            guards,
            player: Combatant::ref_with_hp(100)
        }
    }

    pub fn check_guard_state(&mut self, pos: Position) {
        if self.guard_ref_at(pos).borrow().hp <= 0 {
            self.dungeon[pos.1 as usize][pos.0 as usize] = '.';
        }
    }

    pub fn guard_ref_at(&self, pos: Position) -> CombatantRef {
        self.guards.get(&pos).unwrap().clone()
    }

    pub fn player_ref(&self) -> CombatantRef {
        self.player.clone()
    }

    pub fn resolve_combat_at(&mut self, guard_pos: Position) {
        let guard_ref = self.guard_ref_at(guard_pos);

        self.player.borrow().attack_simple(&guard_ref, 10);
        if guard_ref.borrow().hp <= 0 {
            self.dungeon[guard_pos.1 as usize][guard_pos.0 as usize] = '.';
        }
    }
}