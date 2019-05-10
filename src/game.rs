use std::cell::RefCell;
use std::cmp::{max, min};
use std::io::Write;
use std::rc::Rc;

use console::Key;

use crate::dungeon_provider::DungeonProvider;
use crate::types::{Dungeon, Position};

pub struct Game {
    dungeon: Dungeon,
    player_position: Position,
    camera_offset: i32
}

impl Game {
    pub fn new(dungeon: Dungeon, player_position: Position, camera_offset: i32) -> Game {
        Game { dungeon, player_position, camera_offset }
    }

    pub fn with_dungeon_provider(
        provider: &Rc<RefCell<dyn Iterator<Item=Dungeon>>>,
        player_position: Position,
        camera_offset: i32) -> Game
    {
        Game { dungeon: Rc::clone(provider).borrow_mut().next().unwrap(), player_position, camera_offset }
    }


    pub fn render(&self, writer: &mut Write) -> std::io::Result<(u32)> {
        let player_y = self.player_position.1;
        let start_y = max(0, player_y as i32 - self.camera_offset) as usize;
        let end_y = min(self.dungeon.len() - 1, player_y as usize + self.camera_offset as usize);

        for (y, row) in self.dungeon[start_y..end_y + 1].iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_y as u32) == self.player_position {
                    writer.write(b"@")?;
                } else {
                    if *col == 0 { writer.write(b".")?; }
                    if *col == 1 { writer.write(b"#")?; }
                }
            }

            writer.write(b"\n")?;
        }

        Ok(end_y as u32 - start_y as u32 + 1)
    }

    pub fn on_key(&mut self, key: Key) {
        let x = self.player_position.0 as usize;
        let y = self.player_position.1 as usize;
        match key {
            Key::ArrowLeft => {
                if self.dungeon[y][x - 1] == 0 {
                    self.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                if self.dungeon[y][x + 1] == 0 {
                    self.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                if self.dungeon[y + 1][x] == 0 {
                    self.player_position.1 += 1;
                }
            }
            _ => {}
        }
    }
}
