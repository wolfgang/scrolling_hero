use console::{Term};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    term.write_line("        @")?;
    term.write_line("########.##########")?;
    term.write_line("###.......#########")?;
    term.write_line("######.......######")?;

    Ok(())
}
    