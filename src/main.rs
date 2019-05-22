use std::time::{SystemTime, UNIX_EPOCH};

use console::Term;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use sch::dungeon_generator::dungeon_with_num_paths;
use sch::dungeon_provider::MultiDungeonProvider;
use sch::game::Game;
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut dungeons = Vec::new();

    for _ in 1..50 {
        dungeons.push(generate_dungeon(&mut rng));
    }

    let dungeon_provider = MultiDungeonProvider::shared(dungeons);

    let mut game = Game::new(&dungeon_provider, 2);

    while game.is_running() {
        let num_lines = game.render(&mut term)?;
        game.on_key(term.read_key().unwrap());
        term.clear_last_lines(num_lines as usize)?;
    }

    term.write_line("Thanks for playing!")?;

    Ok(())
}

fn generate_dungeon(rng: &mut StdRng) -> DungeonDefinition {
    let mut dungeon = dungeon_with_num_paths(5, 16, 4, rng);
    let width = dungeon[0].len();
    let height = dungeon.len();
    let exit_position = rng.gen_range(1, width as u32 - 2) as usize;
    let player_position = rng.gen_range(1, width as u32 - 2);
    dungeon[height - 1][exit_position] = 'E';
    (dungeon, (player_position, 0))
}
