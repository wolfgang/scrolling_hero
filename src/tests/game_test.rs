use std::io::{Cursor, Write};
use std::str;

struct Game {
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

#[test]
fn renders_dungeon_and_player() {
    let dungeon = vec![
        vec![1, 0, 1, 1],
        vec![1, 0, 0, 1],
        vec![1, 1, 0, 1]
    ];

    let game = Game::new(dungeon, (2, 1));

    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();

    assert_written_to(&buffer, "#.##\n#.@#\n##.#\n")
}


fn assert_written_to(buffer: &Cursor<Vec<u8>>, written: &str) {
    let reference = buffer.get_ref();
    assert_eq!(written, str::from_utf8(reference).unwrap());
}
