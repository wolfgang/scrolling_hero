
use std::rc::Rc;
use std::cell::{RefCell};
use console::{Key, Term};
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

    dungeon_renderer.render(&mut term, 0, 3)?;

    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => { return Ok(()) }
            Key::ArrowLeft => { player_pos.borrow_mut().0 -= 1 }
            Key::ArrowRight => { player_pos.borrow_mut().0 += 1 }
            Key::ArrowDown => { player_pos.borrow_mut().1 += 1 }
            _ => {}
        }

        term.clear_last_lines(3)?;
        dungeon_renderer.render(&mut term, 0, 3)?;
    }
    
}
    