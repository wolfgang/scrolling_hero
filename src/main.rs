use std::time::{SystemTime, UNIX_EPOCH};

use console::Term;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use sch::dungeon::decorator;
use sch::dungeon::generator::{dungeon_with_num_paths, DungeonGenOpts};
use sch::dungeon::provider::MultiDungeonProvider;
use sch::game::{Game, GameConfig};
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut dungeons = Vec::new();

    for _ in 1..50 {
        dungeons.push(generate_dungeon(60, 40, &mut rng));
    }

    let dungeon_provider = MultiDungeonProvider::shared(dungeons);

    let mut game = Game::with_config(&GameConfig::with_camera_offset(2), &dungeon_provider);


    while game.is_running() {
        let num_lines = game.render(&mut term)?;
        game.on_key(term.read_key().unwrap());
        term.clear_last_lines(num_lines as usize)?;
    }

    term.write_line("Thanks for playing!")?;

    Ok(())
}

fn generate_dungeon(width: usize, height: usize, rng: &mut StdRng) -> DungeonDefinition {
    let opts = DungeonGenOpts {
        width,
        height,
        num_paths: (width / 10) as u8,
        vertical_bias: 1,
        horizontal_bias: 2,
    };

    let mut dungeon = dungeon_with_num_paths(&opts, rng);
    decorator::add_guards(&mut dungeon, 10, rng);

    let width = dungeon[0].len();
    let player_position = rng.gen_range(1, width as u32 - 2);

    (dungeon, (player_position, 0))
}
