use console::{Term};
use sch::dungeon_renderer::render_dungeon;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon = vec![
        vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1]
    ];

    render_dungeon(&dungeon, (8, 0), &mut term)?;

    Ok(())
}
    