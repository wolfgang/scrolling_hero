use core::borrow::Borrow;
use std::rc::Rc;

use console::Term;

use sch::dungeon_renderer::DungeonRenderer;
use sch::move_predicate::MovePredicate;
use sch::player::move_predicates::{NonCollidingPlayerMovePredicate, WallCollidingPlayerMovePredicate, CompositePlayerMovePredicate};
use sch::player::Player;
use sch::player_controller::PlayerController;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon = vec![
        vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0],
        vec![1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,0,0,1,1,0,1,1,1,1,1,1],
        vec![1,1,1,0,0,0,0,0,0,0,1,1,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,0,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1],
    ];

    let max_x = dungeon[0].len() as u32 - 1;
    let max_y = dungeon.len() as u32 - 1;

    let dungeon_ref = Rc::new(dungeon);

    let predicate1: Rc<MovePredicate> = Rc::new(NonCollidingPlayerMovePredicate::new(max_x, max_y));
    let predicate2: Rc<MovePredicate> = Rc::new(WallCollidingPlayerMovePredicate::new(&dungeon_ref));

    let mut predicate = CompositePlayerMovePredicate::new();
    predicate.add(&predicate1);
    predicate.add(&predicate2);

    let player = Player::new(8, 0, &(Rc::new(predicate) as Rc<MovePredicate>));

    let dungeon_renderer = DungeonRenderer::new(dungeon_ref.borrow(), &player);

    let player_controller = PlayerController::new(&player);

    let camera_offset = 2;

    loop {
        let rendered_lines = dungeon_renderer.render(
            &mut term,
            player.position().1 as i32 - camera_offset as i32,
            player.position().1 + camera_offset
            ).unwrap();
        
        let key = term.read_key().unwrap();

        if !player_controller.on_key(key) { return Ok(()) }

        term.clear_last_lines(rendered_lines)?;
    }
    
}
    