use std::cmp::{max, min};
use std::io::Write;

use console::Key;

pub struct Game {
    dungeon: Vec<Vec<u16>>,
    player_position: (u32, u32),
}

impl Game {
    pub fn new(dungeon: Vec<Vec<u16>>, player_position: (u32, u32)) -> Game {
        Game { dungeon, player_position }
    }


    pub fn render(&self, writer: &mut Write) -> std::io::Result<(u32)> {
        let start_y = max(0, self.player_position.1 as i32 - 1) as usize;
        let end_y = min(self.dungeon.len() - 1, self.player_position.1 as usize + 1);


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
        match key {
            Key::ArrowDown => { self.player_position.1 += 1; }
            Key::ArrowLeft => { self.player_position.0 -= 1; }
            Key::ArrowRight => { self.player_position.0 += 1; }
            _ => {}
        }
    }
}
