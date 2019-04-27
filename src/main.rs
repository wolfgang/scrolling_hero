use std::io::Write;
use console::{Term};
use crossterm_cursor::{cursor};

use sch::dungeon_renderer::DungeonRenderer;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let mut cursor = cursor();

    let dungeon = vec![
        vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1]
    ];

    let dungeon_renderer = DungeonRenderer::new(&dungeon);

    dungeon_renderer.render(&mut term)?;

    cursor.save_position()?;
    cursor.move_right(8);
    cursor.move_up(2);
    term.write(b"@")?;
    cursor.reset_position()?;
    
    Ok(())
}
    