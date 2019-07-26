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
    guard_in_combat: Option<Position>
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
            guard_in_combat: None
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

    pub fn resolve_combat(&mut self, pos: Position, dice_roller: &mut dyn DiceRoller) -> ((u8, bool), (u8, bool)) {
        self.guard_in_combat = Some(pos);
        let player_result = self.attack_guard(dice_roller);
        let guard_result = self.attack_player(dice_roller);

        self.check_guard_state(pos);
        (player_result, guard_result)
    }

    pub fn heal_player(&mut self, dice_roller: &mut dyn DiceRoller) -> u8 {
        let heal = self.player.borrow_mut().heal(dice_roller);
        let (x, y) = self.player_position;
        self.dungeon[y as usize][x as usize] = '.';
        heal
    }

    fn attack_guard(&mut self, dice_roller: &mut dyn DiceRoller) -> (u8, bool) {
        self.borrow_player().attack(&self.guard_ref_at(self.guard_in_combat()), dice_roller)
    }

    pub fn attack_player(&mut self, dice_roller: &mut dyn DiceRoller) -> (u8, bool) {
        self.borrow_guard_at(self.guard_in_combat()).attack(&self.player_ref(), dice_roller)
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

    pub fn attempt_player_move_to(&mut self, x_offset: i32, y_offset: u32) {
        if !self.obstacle_at(x_offset, y_offset) {
            self.move_player(x_offset, y_offset);
        }
    }

    pub fn neighbor_at(&self, x_offset: i32, y_offset: u32) -> Option<(Position, char)> {
        let (x, y) = self.player_position;
        let neighbor_x = (x as i32 + x_offset) as usize;
        let neighbor_y = (y + y_offset) as usize;
        if neighbor_x < self.dungeon[0].len() && neighbor_y < self.dungeon.len() {
            return Some(((neighbor_x as u32, neighbor_y as u32), self.dungeon[neighbor_y][neighbor_x]));
        }
        None
    }

    pub fn is_combat_active(&self) -> bool {
        self.guard_in_combat != None
    }

    pub fn end_combat(&mut self) {
        self.guard_in_combat = None;
    }

    pub fn hp_of_guard_in_combat(&self) -> i16 {
        match self.guard_in_combat {
            Some(pos) => { return self.borrow_guard_at(pos).hp }
            None => 0
        }
    }

    fn guard_in_combat(&self) -> Position {
        self.guard_in_combat.unwrap()
    }

    fn obstacle_at(&self, x_offset: i32, y_offset: u32) -> bool {
        match self.neighbor_at(x_offset, y_offset) {
            Some((_, tile)) => { return tile == '#' || tile == 'G'; }
            None => { return true; }
        }
    }

    fn move_player(&mut self, offset_x: i32, offset_y: u32) {
        self.player_position.0 = (self.player_position.0 as i32 + offset_x) as u32;
        self.player_position.1 += offset_y;
    }


    fn check_guard_state(&mut self, pos: Position) {
        if self.borrow_guard_at(pos).hp <= 0 {
            self.dungeon[pos.1 as usize][pos.0 as usize] = '.';
            self.end_combat();

        }
    }
}