use console::{Term};
use sh::dungeon_renderer::render_dungeon;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon = vec![
        "########.##########",
        "###.......#########",
        "######.......######"
    ];

    term.write_line("        @")?;

    render_dungeon(&dungeon, &mut term)?;

    Ok(())
}
    