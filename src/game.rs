use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use console::Key;

use crate::combat;
use crate::combat::Combatant;
use crate::dungeon::renderer::DungeonRenderer;
use crate::dungeon_provider::DungeonProvider;
use crate::types::{DungeonLayout, Position};

pub struct GameState {
    pub dungeon: DungeonLayout,
    pub player_position: Position,
    pub guards: HashMap<(usize, usize), Combatant>,
    player: Combatant,
}

pub struct Game {
    game_state: GameState,
    dungeon_provider: Rc<RefCell<DungeonProvider>>,
    is_running: bool,
    steps: u32,
    dungeon_renderer: DungeonRenderer,
}

impl Game {
    pub fn new(
        provider: &Rc<RefCell<DungeonProvider>>,
        camera_offset: i32) -> Game
    {
        let dungeon_provider = Rc::clone(provider);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();
        Game {
            game_state: GameState {
                dungeon,
                player_position,
                guards: HashMap::new(),
                player: Combatant { hp: 100 },
            },
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
        self.dungeon_renderer.render(
            writer,
            &self.game_state.dungeon,
            &self.game_state.player_position,
            self.steps,
            self.game_state.player.hp)
    }

    pub fn on_key(&mut self, key: Key) {
        let prev_position = self.game_state.player_position;
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

        if prev_position != self.game_state.player_position { self.steps += 1; }
        if self.under_player() == 'E' { self.goto_next_dungeon(); }
    }

    fn under_player(&self) -> char {
        self.neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: i32) {
        match self.neighbor_at(x_offset, y_offset) {
            Some((pos, tile)) => {
                if tile == 'G' {
                    let mut guard = self.game_state.guards.entry(pos).or_insert(Combatant { hp: 20 });
                    combat::resolve_simple(&mut self.game_state.player, &mut guard);
                    if guard.hp <= 0 {
                        self.game_state.dungeon[pos.1][pos.0] = '.';
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
                self.steps = 0;
            }
            None => { self.is_running = false; }
        }
    }
}
