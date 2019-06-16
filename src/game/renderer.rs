use std::cmp::{max, min};
use std::io::{Cursor, Seek, SeekFrom, Write};

use super::state::GameState;

pub struct GameRenderer {
    camera_offset: i32,
    render_buffer: Cursor<Vec<u8>>,
}

impl GameRenderer {
    pub fn new(
        camera_offset: i32) -> GameRenderer
    {
        GameRenderer {
            camera_offset,
            render_buffer: Cursor::new(Vec::with_capacity(512)),
        }
    }

    pub fn render(
        &mut self,
        writer: &mut Write,
        game_state: &GameState,
        messages: &Vec<String>,
    ) -> std::io::Result<(u32)>
    {
        let player_y = game_state.player_position.1;
        let start_y = max(0, player_y as i32 - self.camera_offset) as usize;
        let end_y = min(game_state.dungeon.len() - 1, player_y as usize + self.camera_offset as usize);

        self.clear_render_buffer()?;

        for (y, row) in game_state.dungeon[start_y..end_y + 1].iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, col) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_y as u32) == game_state.player_position {
                    row_str.push('@');
                } else {
                    row_str.push(*col);
                }
            }

            self.render_buffer.write(row_str.as_bytes())?;

            if y == 0 {
                let player_health = game_state.borrow_player().hp;
                self.render_buffer.write(format!("  HP: {}", player_health).as_bytes())?;
            }

            self.render_message_at(y as i32 - 1, &messages);
            self.render_buffer.write(b"\n")?;
        }

        writer.write(self.render_buffer.get_ref())?;
        Ok(end_y as u32 - start_y as u32 + 1)
    }

    fn render_message_at(&mut self, index: i32, messages: &Vec<String>) {
        if index >= 0 && index < messages.len() as i32 {
            self.render_buffer.write(format!("  {}", messages[index as usize]).as_bytes()).unwrap();
        }
    }

    fn clear_render_buffer(&mut self) -> std::io::Result<()> {
        self.render_buffer.get_mut().clear();
        self.render_buffer.seek(SeekFrom::Start(0))?;
        Ok(())
    }
}
