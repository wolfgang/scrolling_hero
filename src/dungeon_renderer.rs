use std::io::{Write};

pub struct DungeonRenderer<'a> {
    dungeon: &'a Vec<Vec<u16>>,
    player_pos: &'a (u32, u32)
}

impl<'a> DungeonRenderer<'a> {
    pub fn new(dungeon: &'a Vec<Vec<u16>>, player_pos: &'a (u32, u32)) -> DungeonRenderer<'a> {
        DungeonRenderer {dungeon: dungeon, player_pos: player_pos}
    }
    pub fn render(&self, writer: &mut Write) -> std::io::Result<()> {
        for (y, row) in self.dungeon.iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, cell) in row.iter().enumerate() {
                if (x as u32, y as u32) == *self.player_pos {
                    row_str.push('@');
                }
                else {
                    if *cell == 0 { row_str.push('.')}
                    if *cell == 1 { row_str.push('#')}                
                }
            }

            writer.write_fmt(format_args!("{}\n", &row_str))?;
        }

        Ok(())
    }
}
