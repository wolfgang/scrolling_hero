
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
        vec![1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1],
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

    let camera_offset = 2;
    let wanted_lines = 2*camera_offset + 1;

    loop {
        let rendered_lines = dungeon_renderer.render(
            &mut term, 
            player_pos.borrow().1 as i32 - camera_offset as i32, 
            wanted_lines as u32).unwrap();
        
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
            }
            _ => {}
        }

        term.clear_last_lines(rendered_lines)?;
    }
    
}
    