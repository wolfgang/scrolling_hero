use std::io::{Write};

pub struct DungeonRenderer {
}

impl DungeonRenderer {
    pub fn new() -> DungeonRenderer {
        DungeonRenderer {}
    }
    pub fn render(&self, dungeon: &Vec<Vec<u16>>, player_pos: (u32, u32), writer: &mut Write) -> std::io::Result<()> {
        for (y, row) in dungeon.iter().enumerate() {
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
