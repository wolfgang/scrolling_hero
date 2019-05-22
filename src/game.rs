use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use console::Key;

use crate::dungeon_provider::DungeonProvider;
use crate::dungeon_renderer::DungeonRenderer;
use crate::types::{DungeonLayout, Position};

pub struct Game {
    dungeon: DungeonLayout,
    player_position: Position,
    dungeon_provider: Rc<RefCell<DungeonProvider>>,
    is_running: bool,
    steps: u32,
    dungeon_renderer: DungeonRenderer
}

impl Game {
    pub fn new(
        provider: &Rc<RefCell<DungeonProvider>>,
        camera_offset: i32) -> Game
    {
        let dungeon_provider = Rc::clone(provider);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();
        Game {
            dungeon,
            player_position,
            dungeon_provider,
            is_running: true,
            steps: 0,
            dungeon_renderer: DungeonRenderer::new(camera_offset)
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }


    pub fn render(&mut self, writer: &mut Write) -> std::io::Result<(u32)> {
        self.dungeon_renderer.render(writer, &self.dungeon, &self.player_position, self.steps)
    }

    pub fn on_key(&mut self, key: Key) {
        let prev_position = self.player_position;
        match key {
            Key::ArrowLeft => {
                if self.relative_to_player(-1, 0) != '#' {
                    self.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                if self.relative_to_player(1, 0) != '#' {
                    self.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                let is_at_bottom = self.player_position.1 == self.dungeon.len() as u32 - 1;
                if !is_at_bottom && self.relative_to_player(0, 1) != '#' {
                    self.player_position.1 += 1;
                }
            }
            Key::Escape => {
                self.is_running = false;
            }
            _ => {}
        }

        if prev_position != self.player_position { self.steps += 1; }
        if self.under_player() == 'E' { self.goto_next_dungeon(); }
    }

    fn under_player(&self) -> char {
        self.relative_to_player(0, 0)
    }

    fn relative_to_player(&self, x_offset: i32, y_offset: i32) -> char {
        let x = self.player_position.0 as i32;
        let y = self.player_position.1 as i32;
        self.dungeon[(y + y_offset) as usize][(x + x_offset) as usize]
    }

    fn goto_next_dungeon(&mut self) {
        match self.dungeon_provider.borrow_mut().next() {
            Some((next_dungeon, next_player_pos)) => {
                self.dungeon = next_dungeon;
                self.player_position = next_player_pos;
                self.steps = 0;
            }
            None => { self.is_running = false; }
        }
    }
}
