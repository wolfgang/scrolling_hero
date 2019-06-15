use std::collections::HashMap;

use crate::game::dice_roller::DiceRoller;
use crate::types::{CombatantRef, DungeonLayout, Position};

use super::combat::Combatant;

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    guards: HashMap<Position, CombatantRef>,
    player: CombatantRef,
}

impl GameState {
    pub fn new(dungeon: DungeonLayout, player_position: Position, guard_hp: u16) -> GameState {
        let mut guards = HashMap::new();

        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    guards.insert((x as u32, y as u32), Combatant::ref_with_hp(guard_hp as i16));
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

    pub fn resolve_combat(&mut self, pos: Position, dice_roller: &mut DiceRoller) {
        let guard_ref = self.guard_ref_at(pos);
        let player_ref = self.player_ref();

        player_ref.borrow().attack(&guard_ref, dice_roller);
        guard_ref.borrow().attack(&player_ref, dice_roller);

        self.check_guard_state(pos);

    }


    pub fn player_ref(&self) -> CombatantRef {
        self.player.clone()
    }

    fn guard_ref_at(&self, pos: Position) -> CombatantRef {
        self.guards.get(&pos).unwrap().clone()
    }

    fn check_guard_state(&mut self, pos: Position) {
        if self.guard_ref_at(pos).borrow().hp <= 0 {
            self.dungeon[pos.1 as usize][pos.0 as usize] = '.';
        }
    }
}