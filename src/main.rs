use console::{Term};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    write(&mut term, b"        @\n")?;
    write(&mut term, b"########.##########\n")?;
    write(&mut term, b"###.......#########\n")?;
    write(&mut term, b"######.......######\n")?;

    Ok(())
}

fn write<W: Write>(writer: &mut W, buf: &[u8]) -> std::io::Result<()> {
    writer.write(buf)?;
    Ok(())
}

    