use std::cell::RefCell;
use std::cmp::{max, min};
use std::io::{Cursor, Write};
use std::rc::Rc;

use console::Key;

use crate::dungeon_provider::DungeonProvider;
use crate::types::{DungeonLayout, Position};

pub struct Game {
    dungeon: DungeonLayout,
    player_position: Position,
    camera_offset: i32,
    dungeon_provider: Rc<RefCell<DungeonProvider>>,
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

        let mut buffer = Cursor::new(Vec::new());
        for (y, row) in self.dungeon[start_y..end_y + 1].iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_y as u32) == self.player_position {
                    buffer.write(b"@")?;
                } else {
                    if *col == 0 { buffer.write(b".")?; }
                    if *col == 1 { buffer.write(b"#")?; }
                    if *col == 2 { buffer.write(b"E")?; }
                }
            }

            buffer.write(b"\n")?;
        }

        writer.write(buffer.get_ref());
        Ok(end_y as u32 - start_y as u32 + 1)
    }

    pub fn on_key(&mut self, key: Key) {
        match key {
            Key::ArrowLeft => {
                if self.relative_to_player(-1, 0) != 1 {
                    self.player_position.0 -= 1;
                }
            }
            Key::ArrowRight => {
                if self.relative_to_player(1, 0) != 1 {
                    self.player_position.0 += 1;
                }
            }
            Key::ArrowDown => {
                if self.relative_to_player(0, 1) != 1 {
                    self.player_position.1 += 1;
                }
            }
            _ => {}
        }

        if self.under_player() == 2 { self.goto_next_dungeon(); }
    }

    fn under_player(&self) -> u16 {
        self.relative_to_player(0, 0)
    }

    fn relative_to_player(&self, x_offset: i32, y_offset: i32) -> u16 {
        let x = self.player_position.0 as i32;
        let y = self.player_position.1 as i32;
        self.dungeon[(y + y_offset) as usize][(x + x_offset) as usize]
    }

    fn goto_next_dungeon(&mut self) {
        let (next_dungeon, next_player_pos) = self.dungeon_provider.borrow_mut().next().unwrap();
        self.dungeon = next_dungeon;
        self.player_position = next_player_pos;
    }
}
