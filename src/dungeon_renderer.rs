use std::io::{Write};

pub struct DungeonRenderer<'a> {
    dungeon: &'a Vec<Vec<u16>>
}

impl<'a> DungeonRenderer<'a> {
    pub fn new(dungeon: &Vec<Vec<u16>>) -> DungeonRenderer {
        DungeonRenderer {dungeon: dungeon}
    }
    pub fn render(&self, writer: &mut Write) -> std::io::Result<()> {
        for row in self.dungeon {
            let mut row_str = String::with_capacity(row.len());
            for cell in row {
                if *cell == 0 { row_str.push('.')}
                if *cell == 1 { row_str.push('#')}                
            }

            writer.write_fmt(format_args!("{}\n", &row_str))?;
        }

        Ok(())
    }
}
