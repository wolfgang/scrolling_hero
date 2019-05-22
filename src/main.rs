use std::time::{SystemTime, UNIX_EPOCH};

use console::Term;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use sch::dungeon_generator::dungeon_with_one_path;
use sch::dungeon_helpers::make_dungeon;
use sch::dungeon_provider::MultiDungeonProvider;
use sch::game::Game;
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let dungeon_provider = MultiDungeonProvider::shared(vec![dungeon1(), dungeon2()]);

    let mut game = Game::new(&dungeon_provider, 2);

    while game.is_running() {
        let num_lines = game.render(&mut term)?;
        game.on_key(term.read_key().unwrap());
        term.clear_last_lines(num_lines as usize)?;
    }

    term.write_line("Thanks for playing!")?;

    Ok(())
}

fn dungeon1() -> DungeonDefinition {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut dungeon = dungeon_with_one_path(32, 16, &mut rng);
    let width = dungeon[0].len();
    let height = dungeon.len();
    dungeon[height - 1][rng.gen_range(1, width as u32 - 2) as usize] = 'E';
    (dungeon, (rng.gen_range(1, width as u32 - 2), 0))
}

fn dungeon2() -> DungeonDefinition {
    make_dungeon(vec![
        "#########################################",
        "######################......#############",
        "#############...........@...#############",
        "#############.#######.......#############",
        "##########....#####......##.#############",
        "############.............##....##########",
        "###############..........#####.##########",
        "#################.####...#####.##########",
        "#################.......###.....#########",
        "##################....E.....#############",
        "#########################################",
    ])
}