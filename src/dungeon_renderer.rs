use std::cmp::{max, min};
use std::io::Write;

use crate::player::Player0;

pub struct DungeonRenderer<'a> {
    dungeon: &'a Vec<Vec<u16>>,
    player: &'a Player0
}

impl<'a> DungeonRenderer<'a> {
    pub fn new(
        dungeon: &'a Vec<Vec<u16>>,
        player: &'a Player0,
    ) -> DungeonRenderer<'a>
    {
        DungeonRenderer { dungeon, player }
    }
    pub fn render(&self, writer: &mut Write, from: i32, to: u32) -> std::io::Result<usize> {
        let start_row = max(from, 0) as usize;
        let end_row = min(self.dungeon.len() - 1, to as usize);
        for (y, row) in self.dungeon[start_row..end_row + 1].iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, cell) in row.iter().enumerate() {
                if (x as u32, y as u32 + start_row as u32) == self.player.position() {
                    row_str.push('@');
                }
                else {
                    if *cell == 0 { row_str.push('.')}
                    if *cell == 1 { row_str.push('#')}                
                }
            }

            writer.write_fmt(format_args!("{}\n", &row_str))?;
        }

        Ok(end_row - start_row + 1)
    }
}
