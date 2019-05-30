use std::cmp::{max, min};
use std::io::{Cursor, Seek, SeekFrom, Write};

use crate::types::{DungeonLayout, Position};

pub struct DungeonRenderer {
    camera_offset: i32,
    render_buffer: Cursor<Vec<u8>>,
}

impl DungeonRenderer {
    pub fn new(
        camera_offset: i32) -> DungeonRenderer
    {
        DungeonRenderer {
            camera_offset,
            render_buffer: Cursor::new(Vec::with_capacity(512)),
        }
    }

    pub fn render(
        &mut self,
        writer: &mut Write,
        dungeon: &DungeonLayout,
        player_position: &Position,
        steps: u32,
        in_combat: bool,
        player_health: u32
    ) -> std::io::Result<(u32)>
    {
        let player_y = player_position.1;
        let start_y = max(0, player_y as i32 - self.camera_offset) as usize;
        let end_y = min(dungeon.len() - 1, player_y as usize + self.camera_offset as usize);

        self.clear_render_buffer()?;

        for (y, row) in dungeon[start_y..end_y + 1].iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, col) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_y as u32) == *player_position {
                    row_str.push('@');
                } else {
                    row_str.push(*col);
                }
            }

            self.render_buffer.write(row_str.as_bytes())?;

            if y == 0 {
                self.render_buffer.write(format!("  Steps: {}", steps).as_bytes())?;
            }

            if y == 1 {
                self.render_buffer.write(format!("  HP: {}", player_health).as_bytes())?;
            }
            if in_combat && y == 2 {
                self.render_buffer.write(format!("  GHP: {}", 20).as_bytes())?;
            }


            self.render_buffer.write(b"\n")?;
        }

        writer.write(self.render_buffer.get_ref())?;
        Ok(end_y as u32 - start_y as u32 + 1)
    }

    fn clear_render_buffer(&mut self) -> std::io::Result<()> {
        self.render_buffer.get_mut().clear();
        self.render_buffer.seek(SeekFrom::Start(0))?;
        Ok(())
    }
}
