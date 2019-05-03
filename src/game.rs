use std::io::Write;

pub struct Game {
    dungeon: Vec<Vec<u16>>,
    player_position: (u32, u32),
}

impl Game {
    pub fn new(dungeon: Vec<Vec<u16>>, player_position: (u32, u32)) -> Game {
        Game { dungeon, player_position }
    }


    pub fn render(&self, writer: &mut Write) -> std::io::Result<()> {
        for (y, row) in self.dungeon.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if (x as u32, y as u32) == self.player_position {
                    writer.write(b"@")?;
                } else {
                    if *col == 0 { writer.write(b".")?; }
                    if *col == 1 { writer.write(b"#")?; }
                }
            }

            writer.write(b"\n")?;
        }

        Ok(())
    }
}
