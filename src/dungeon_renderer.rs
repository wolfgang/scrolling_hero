use std::io::{Write};

pub struct DungeonRenderer<'a> {
    dungeon: &'a Vec<Vec<u16>>
}

impl<'a> DungeonRenderer<'a> {
    pub fn new(dungeon: &Vec<Vec<u16>>) -> DungeonRenderer {
        DungeonRenderer {dungeon: dungeon}
    }
    pub fn render(&self, player_pos: (u32, u32), writer: &mut Write) -> std::io::Result<()> {
        for (y, row) in self.dungeon.iter().enumerate() {
            let mut row_str = String::with_capacity(row.len());
            for (x, cell) in row.iter().enumerate() {
                if (x as u32, y as u32) == player_pos { row_str.push('@') }
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
