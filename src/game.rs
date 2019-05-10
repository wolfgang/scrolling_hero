use std::cell::RefCell;
use std::cmp::{max, min};
use std::io::Write;
use std::rc::Rc;

use console::Key;

use crate::dungeon_provider::DungeonProvider;
use crate::types::{DungeonLayout, Position};

pub struct Game {
    dungeon: DungeonLayout,
    player_position: Position,
    camera_offset: i32,
    dungeon_provider: Rc<RefCell<DungeonProvider>>
}

impl Game {
    pub fn new(
        provider: &Rc<RefCell<DungeonProvider>>,
        camera_offset: i32) -> Game
    {
        let (dungeon, player_position) = Rc::clone(provider).borrow_mut().next().unwrap();
        Game { dungeon, player_position, camera_offset, dungeon_provider: Rc::clone(provider) }
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
                    if *col == 2 { writer.write(b"E")?; }
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
                if self.dungeon[y][x - 1] != 1 {
                    self.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                if self.dungeon[y][x + 1] != 1 {
                    self.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                if self.dungeon[y + 1][x] != 1 {
                    self.player_position.1 += 1;
                }
            }
            _ => {}
        }

        if self.dungeon[self.player_position.1 as usize][self.player_position.0 as usize] == 2 {
            let (next_dungeon, next_player_pos) = self.dungeon_provider.borrow_mut().next().unwrap();
            self.dungeon = next_dungeon;
            self.player_position = next_player_pos;
        }
    }
}
