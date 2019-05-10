use std::cmp::{max, min};
use std::io::Write;

use console::Key;

use crate::dungeon_provider::DungeonProvider;

pub struct Game {
    dungeon: Vec<Vec<u16>>,
    player_position: (u32, u32),
    camera_offset: i32
}

impl Game {
    pub fn new(dungeon: Vec<Vec<u16>>, player_position: (u32, u32), camera_offset: i32) -> Game {
        Game { dungeon, player_position, camera_offset }
    }

    pub fn with_dungeon_provider(
        provider: &impl DungeonProvider,
        player_position: (u32, u32),
        camera_offset: i32) -> Game
    {
        Game { dungeon: provider.next().clone(), player_position, camera_offset }
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
