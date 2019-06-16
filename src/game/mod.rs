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
    game_renderer: GameRenderer,
    dice_roller: Box<dyn DiceRoller>,
    messages: Vec<String>
}

impl Game {
    pub fn with_config(config: &GameConfig, dungeon_provider_ref: &DungeonProviderRef) -> Game {
        let dungeon_provider = Rc::clone(dungeon_provider_ref);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();

        let mut game = Game {
            game_state: GameState::from_game_config(config, dungeon, player_position),
            dungeon_provider,
            game_renderer: GameRenderer::new(config.camera_offset),
            dice_roller: Box::from(RandomizedDiceRoller::new()),
            is_running: true,
            messages: Vec::with_capacity(10)
        };

        game.show_player_hp();

        game
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn render(&mut self, writer: &mut Write) -> std::io::Result<(u32)> {
        self.game_renderer.render(
            writer,
            &self.game_state,
            &self.messages
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
                self.show_player_hp();
                if tile == 'G' {
                    let (damage_to_guard, damage_to_player) = self.game_state.resolve_combat(pos, &mut *self.dice_roller);

                    self.show_player_hp();
                    self.show_combat_messages(pos, damage_to_guard, damage_to_player);

                    if self.game_state.borrow_player().hp <= 0 {
                        self.is_running = false;
                    }

                }
            }

            None => {}
        }
    }

    fn show_player_hp(&mut self) {
        self.messages.clear();
        let player_health = self.game_state.borrow_player().hp;
        self.messages.push(Game::player_health_message(player_health));
    }

    fn show_combat_messages(&mut self, guard_pos: (u32, u32), damage_to_guard: u8, damage_to_player: u8) {
        let guard_health = self.game_state.borrow_guard_at(guard_pos).hp;
        let player_health = self.game_state.borrow_player().hp;
        self.messages.push(Game::attack_message("Player", "Guard", damage_to_guard, player_health));
        self.messages.push(Game::attack_message("Guard", "Player", damage_to_player, guard_health));
    }

    fn attack_message(attacker: &str, target: &str, damage: u8, attacker_hp: i16) -> String {
        if attacker_hp <= 0 {
            return String::from(format!("{} dies!", attacker))
        }
        if damage > 0 {
            return String::from(format!("{} hits {} for {}", attacker, target, damage))
        }
        String::from(format!("{} misses {}!", attacker, target))
    }

    fn player_health_message(player_hp: i16) -> String {
        format!("HP: {}", player_hp)
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
