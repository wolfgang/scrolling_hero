use std::cell::RefCell;
use std::collections::HashMap;
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
    dungeon_renderer: DungeonRenderer,
    guard_states: HashMap<(usize, usize), u32>,
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
            dungeon_renderer: DungeonRenderer::new(camera_offset),
            guard_states: HashMap::new(),
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
                self.process_neighbor(-1, 0);
                if !self.obstacle_at(-1, 0) {
                    self.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                self.process_neighbor(1, 0);
                if !self.obstacle_at(1, 0) {
                    self.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                self.process_neighbor(0, 1);
                if !self.obstacle_at(0, 1) {
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
        self.neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: i32) {
        match self.neighbor_at(x_offset, y_offset) {
            Some((pos, tile)) => {
                if tile == 'G' {
                    let state = self.guard_states.entry(pos).or_insert(0);
                    *state += 1;

                    if *state == 2 {
                        self.dungeon[pos.1][pos.0] = '.';
                    }
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
        let x = self.player_position.0 as i32;
        let y = self.player_position.1 as i32;
        let neighbor_x = (x + x_offset) as usize;
        let neighbor_y = (y + y_offset) as usize;
        if neighbor_x < self.dungeon[0].len() && neighbor_y < self.dungeon.len() {
            return Some(((neighbor_x, neighbor_y), self.dungeon[neighbor_y][neighbor_x]));
        }
        None
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
