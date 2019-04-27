
use std::rc::Rc;
use std::cell::{RefCell};
use console::{Term};
use sch::dungeon_renderer::DungeonRenderer;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon = vec![
        vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1]
    ];

    let player_pos = Rc::new(RefCell::new((8, 1)));
    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player_pos);

    dungeon_renderer.render(&mut term)?;
    
    Ok(())
}
    