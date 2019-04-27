use std::io::{Write};
use std::str;

pub fn render_dungeon(dungeon: &Vec<Vec<u16>>, writer: &mut Write) -> std::io::Result<()> {
    for row in dungeon {
        let mut row_str = String::with_capacity(row.len());
        for cell in row {
            if *cell == 0 { row_str.push('.')}
            if *cell == 1 { row_str.push('#')}
        }

        writer.write_fmt(format_args!("{}\n", &row_str))?;
    }

    Ok(())
}