use std::cell::Ref;
use std::collections::HashMap;

use crate::game::combatant::CombatantConfig;
use crate::game::dice_roller::DiceRoller;
use crate::game::GameConfig;
use crate::types::{CombatantRef, DungeonLayout, Position};

use super::combatant::Combatant;

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    guards: HashMap<Position, CombatantRef>,
    player: CombatantRef,
}

impl GameState {
    pub fn from_game_config(
        game_config: &GameConfig, dungeon: DungeonLayout, player_position: Position) -> GameState {
        let mut guards = HashMap::new();

        let guard_config = CombatantConfig {
            initial_hp: game_config.guard_hp,
            attack: game_config.guard_attack,
            defense: game_config.guard_defense,
        };

        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    let combatant_ref = Combatant::with_config(&guard_config).into_ref();
                    guards.insert((x as u32, y as u32), combatant_ref);
                }
            }
        }

        let player_config = CombatantConfig {
            initial_hp: game_config.player_hp,
            attack: game_config.player_attack,
            defense: game_config.player_defense,
        };

        GameState {
            dungeon,
            player_position,
            guards,
            player: Combatant::with_config(&player_config).into_ref(),
        }
    }

    pub fn resolve_combat(&mut self, pos: Position, dice_roller: &mut DiceRoller) {
        let guard_ref = self.guard_ref_at(pos);
        let player_ref = self.player_ref();

        player_ref.borrow().attack(&guard_ref, dice_roller);
        guard_ref.borrow().attack(&player_ref, dice_roller);

        self.check_guard_state(pos);
    }


    pub fn borrow_player(&self) -> Ref<Combatant> {
        self.player.borrow()
    }

    pub fn player_ref(&self) -> CombatantRef {
        self.player.clone()
    }

    pub fn borrow_guard_at(&self, pos: Position) -> Ref<Combatant> {
        self.guards.get(&pos).unwrap().borrow()
    }

    pub fn guard_ref_at(&self, pos: Position) -> CombatantRef {
        self.guards.get(&pos).unwrap().clone()
    }

    fn check_guard_state(&mut self, pos: Position) {
        if self.borrow_guard_at(pos).hp <= 0 {
            self.dungeon[pos.1 as usize][pos.0 as usize] = '.';
        }
    }
}