
use std::rc::Rc;
use std::cell::{RefCell};
use console::{Key, Term};
use sch::dungeon_renderer::DungeonRenderer;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon = vec![
        vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,0,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1],
    ];

    let player_pos = Rc::new(RefCell::new((8, 0)));
    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player_pos);

    let mut visible_lines = 5;

    term.move_cursor_down(visible_lines)?;

    loop {
        term.clear_last_lines(visible_lines)?;
        dungeon_renderer.render(&mut term, player_pos.borrow().1 as i32 - 2, visible_lines as u32)?;
        let key = term.read_key().unwrap();


        match key {
            Key::Escape => { return Ok(()) }
            Key::ArrowLeft => { player_pos.borrow_mut().0 -= 1 }
            Key::ArrowRight => { 
                player_pos.borrow_mut().0 += 1 
            }
            Key::ArrowDown => { 
                if player_pos.borrow().1 < dungeon.len() as u32 - 1 {
                    player_pos.borrow_mut().1 += 1 
                }
                else {
                    visible_lines = 3;
                }
            }
            _ => {}
        }

    }
    
}
    