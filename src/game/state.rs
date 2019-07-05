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
        game_config: &GameConfig,
        dungeon: DungeonLayout,
        player_position: Position) -> GameState
    {
        GameState {
            player: GameState::create_player(game_config),
            guards: GameState::create_guards(game_config, &dungeon),
            player_position,
            dungeon,
        }
    }

    fn create_player(game_config: &GameConfig) -> CombatantRef {
        let player_config = CombatantConfig {
            initial_hp: game_config.player_hp,
            attack: game_config.player_attack,
            defense: game_config.player_defense,
        };

        Combatant::with_config(&player_config).into_ref()
    }

    fn create_guards(game_config: &GameConfig, dungeon: &Vec<Vec<char>>) -> HashMap<(u32, u32), CombatantRef> {
        let guard_config = CombatantConfig {
            initial_hp: game_config.guard_hp,
            attack: game_config.guard_attack,
            defense: game_config.guard_defense,
        };

        let mut guards = HashMap::new();
        for y in 0..dungeon.len() {
            for x in 0..dungeon[0].len() {
                if dungeon[y][x] == 'G' {
                    let combatant_ref = Combatant::with_config(&guard_config).into_ref();
                    guards.insert((x as u32, y as u32), combatant_ref);
                }
            }
        }
        guards
    }

    pub fn resolve_combat(&mut self, pos: Position, dice_roller: &mut DiceRoller) -> (u8, u8) {
        let guard_ref = self.guard_ref_at(pos);
        let player_ref = self.player_ref();

        let damage_to_guard = player_ref.borrow().attack(&guard_ref, dice_roller);
        let damage_to_player = guard_ref.borrow().attack(&player_ref, dice_roller);

        self.check_guard_state(pos);
        (damage_to_guard, damage_to_player)


    }

    pub fn heal_player(&mut self, dice_roller: &mut DiceRoller) {
        self.player.borrow_mut().heal(dice_roller);
        let (x, y) = self.player_position;
        self.dungeon[y as usize][x as usize] = '.';
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

    pub(crate) fn neighbor_at(&self, x_offset: i32, y_offset: i32) -> Option<(Position, char)> {
        let (x, y) = self.player_position;
        let neighbor_x = (x as i32 + x_offset) as usize;
        let neighbor_y = (y as i32 + y_offset) as usize;
        if neighbor_x < self.dungeon[0].len() && neighbor_y < self.dungeon.len() {
            return Some(((neighbor_x as u32, neighbor_y as u32), self.dungeon[neighbor_y][neighbor_x]));
        }
        None
    }


    fn check_guard_state(&mut self, pos: Position) {
        if self.borrow_guard_at(pos).hp <= 0 {
            self.dungeon[pos.1 as usize][pos.0 as usize] = '.';
        }
    }
}