use std::io::Write;
use std::rc::Rc;

use console::Key;

use renderer::GameRenderer;
use state::GameState;

use crate::game::dice_roller::DiceRoller;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::types::{DungeonProviderRef, Position};

pub mod renderer;
pub mod state;
pub mod combatant;
pub mod dice_roller;
pub mod randomized_dice_roller;

#[derive(Default)]
pub struct GameConfig {
    pub camera_offset: i32,
    pub guard_hp: u16,
    pub guard_attack: u8,
    pub guard_defense: u8,

    pub player_hp: u16,
    pub player_attack: u8,
    pub player_defense: u8,

}

impl GameConfig {
    pub fn with_defaults() -> GameConfig {
        GameConfig { camera_offset: 1, ..Default::default() }
    }
}

pub struct Game {
    pub game_state: GameState,
    dungeon_provider: DungeonProviderRef,
    is_running: bool,
    dungeon_renderer: GameRenderer,
    dice_roller: Box<dyn DiceRoller>,
    message: String
}

impl Game {
    pub fn with_config(config: &GameConfig, dungeon_provider_ref: &DungeonProviderRef) -> Game {
        let dungeon_provider = Rc::clone(dungeon_provider_ref);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();
        Game {
            game_state: GameState::from_game_config(config, dungeon, player_position),
            dungeon_provider,
            dungeon_renderer: GameRenderer::new(config.camera_offset),
            dice_roller: Box::from(RandomizedDiceRoller::new()),
            is_running: true,
            message: String::with_capacity(64)
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }


    pub fn render(&mut self, writer: &mut Write) -> std::io::Result<(u32)> {
        self.dungeon_renderer.render(
            writer,
            &self.game_state,
            &self.message
        )
    }

    pub fn on_key(&mut self, key: Key) {
        match key {
            Key::ArrowLeft => {
                self.process_neighbor(-1, 0);
                if !self.obstacle_at(-1, 0) {
                    self.game_state.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                self.process_neighbor(1, 0);
                if !self.obstacle_at(1, 0) {
                    self.game_state.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                self.process_neighbor(0, 1);
                if !self.obstacle_at(0, 1) {
                    self.game_state.player_position.1 += 1;
                }
            }
            Key::Escape => {
                self.is_running = false;
            }
            _ => {}
        }

        if self.under_player() == 'E' { self.goto_next_dungeon(); }
    }

    fn under_player(&self) -> char {
        self.neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: i32) {
        match self.neighbor_at(x_offset, y_offset) {
            Some((pos, tile)) => {
                if tile == 'G' {
                    self.game_state.resolve_combat(pos, &mut *self.dice_roller);
                    self.message = String::from("Player hits Guard for 1234");
                }
            }

            None => {}
        }
    }


    fn obstacle_at(&self, x_offset: i32, y_offset: i32) -> bool {
        match self.neighbor_at(x_offset, y_offset) {
            Some((_, tile)) => { return tile == '#' || tile == 'G'; }
            None => { return true; }
        }
    }

    fn neighbor_at(&self, x_offset: i32, y_offset: i32) -> Option<(Position, char)> {
        let x = self.game_state.player_position.0 as i32;
        let y = self.game_state.player_position.1 as i32;
        let neighbor_x = (x + x_offset) as usize;
        let neighbor_y = (y + y_offset) as usize;
        if neighbor_x < self.game_state.dungeon[0].len() && neighbor_y < self.game_state.dungeon.len() {
            return Some(((neighbor_x as u32, neighbor_y as u32), self.game_state.dungeon[neighbor_y][neighbor_x]));
        }
        None
    }


    fn goto_next_dungeon(&mut self) {
        match self.dungeon_provider.borrow_mut().next() {
            Some((next_dungeon, next_player_pos)) => {
                self.game_state.dungeon = next_dungeon;
                self.game_state.player_position = next_player_pos;
            }
            None => { self.is_running = false; }
        }
    }
}
