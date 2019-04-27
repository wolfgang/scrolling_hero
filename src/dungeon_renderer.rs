use std::io::{Write};
use std::str;

pub fn render_dungeon(dungeon: &Vec<&str>, writer: &mut Write) -> std::io::Result<()> {
    for line in dungeon {
        writer.write_fmt(format_args!("{}\n", line))?;
    }

    Ok(())
}