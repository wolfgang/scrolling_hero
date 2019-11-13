use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use console::Term;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use sch::dungeon::decorator;
use sch::dungeon::generator::{dungeon_with_num_paths, DungeonGenOpts};
use sch::dungeon::provider::MultiDungeonProvider;
use sch::game::{Game, GameConfig};
use sch::raylib::run_game_in_raylib;
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);

    let args: Vec<String> = env::args().collect();
    let graphic_mode = args.len() == 2 && &args[1] == "-g";

    let game_config = GameConfig {
        camera_offset: if graphic_mode { 5 } else { 2 },
        guard_hp: 20,
        guard_attack: 2,
        guard_defense: 10,
        player_hp: 100,
        player_attack: 5,
        player_defense: 15,
    };

    let dungeon_width = if graphic_mode { 20 } else { 60 };
    let mut dungeons = Vec::new();

    for _ in 1..50 {
        dungeons.push(generate_dungeon(dungeon_width, 40, &mut rng));
    }

    let dungeon_provider = MultiDungeonProvider::shared(dungeons);
    let mut game = Game::with_config(&game_config, &dungeon_provider);

    if graphic_mode {
        return run_game_in_raylib(&mut game, dungeon_width);
    }

    run_game_in_terminal(&mut game)
}

fn run_game_in_terminal(game: &mut Game) -> std::io::Result<()> {
    let mut term = Term::stdout();
    while game.is_running() {
        let num_lines = game.render(&mut term)?;
        game.on_key(term.read_key().unwrap());
        term.clear_last_lines(num_lines as usize)?;
    }
    term.write_line("Thanks for playing!")
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
    decorator::add_health_potions(&mut dungeon, 2, rng);

    let width = dungeon[0].len();
    let player_position = rng.gen_range(1, width as u32 - 2);

    (dungeon, (player_position, 0))
}
