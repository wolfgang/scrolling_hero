use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use console::Key;

use renderer::GameRenderer;
use state::GameState;

use crate::dungeon::provider::DungeonProvider;

pub mod renderer;
pub mod state;
pub mod combat;

pub struct Game {
    game_state: GameState,
    dungeon_provider: Rc<RefCell<DungeonProvider>>,
    is_running: bool,
    dungeon_renderer: GameRenderer,
}

impl Game {
    pub fn new(
        provider: &Rc<RefCell<DungeonProvider>>,
        camera_offset: i32) -> Game
    {
        let dungeon_provider = Rc::clone(provider);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();
        Game {
            game_state: GameState::new(dungeon, player_position),
            dungeon_provider,
            is_running: true,
            dungeon_renderer: GameRenderer::new(camera_offset),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }


    pub fn render(&mut self, writer: &mut Write) -> std::io::Result<(u32)> {
        self.dungeon_renderer.render(
            writer,
            &self.game_state
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
                    combat::resolve_simple(&mut self.game_state, pos);
                    self.game_state.check_guard_state(pos);
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

    fn neighbor_at(&self, x_offset: i32, y_offset: i32) -> Option<((usize, usize), char)> {
        let x = self.game_state.player_position.0 as i32;
        let y = self.game_state.player_position.1 as i32;
        let neighbor_x = (x + x_offset) as usize;
        let neighbor_y = (y + y_offset) as usize;
        if neighbor_x < self.game_state.dungeon[0].len() && neighbor_y < self.game_state.dungeon.len() {
            return Some(((neighbor_x, neighbor_y), self.game_state.dungeon[neighbor_y][neighbor_x]));
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
