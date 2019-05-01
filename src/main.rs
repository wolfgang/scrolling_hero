use console::Term;

use sch::dungeon_renderer::DungeonRenderer;
use sch::player::{Player, Player0};
use sch::player::move_predicates::NonCollidingPlayerMovePredicate;
use sch::player_controller::PlayerController;

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

    let max_x = dungeon[0].len() as u32 - 1;
    let max_y = dungeon.len() as u32 - 1;
    let player0 = Player0::new(8, 0, max_x, max_y);

    let predicate = NonCollidingPlayerMovePredicate::new(max_x, max_y);
    let player = Player::new(8, 0, &predicate);

    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player);

    let player_controller = PlayerController::new(&player0);

    let camera_offset = 2;

    loop {
        let rendered_lines = dungeon_renderer.render(
            &mut term,
            player0.position().1 as i32 - camera_offset as i32,
            player0.position().1 + camera_offset
            ).unwrap();
        
        let key = term.read_key().unwrap();

        if !player_controller.on_key(key) { return Ok(()) }

        term.clear_last_lines(rendered_lines)?;
    }
    
}
    