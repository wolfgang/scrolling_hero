use console::{Key, Term};

use sch::dungeon_helpers::make_dungeon;
use sch::dungeon_provider::MultiDungeonProvider;
use sch::game::Game;
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let dungeon_provider = MultiDungeonProvider::shared(vec![dungeon1(), dungeon2()]);

    let mut game = Game::new(&dungeon_provider, 2);

    loop {
        let num_lines = game.render(&mut term)?;

        let key = term.read_key().unwrap();
        if key == Key::Escape { return Ok(()); }

        game.on_key(key);

        term.clear_last_lines(num_lines as usize)?;
    }
}

fn dungeon1() -> DungeonDefinition {
    make_dungeon(vec![
        "######################.@.################",
        "######################......#############",
        "####################........#############",
        "#####################.......#############",
        "##########...............##.#############",
        "############.............##....##########",
        "###############..........#####.##########",
        "#################.####...#####.##########",
        "#################.......###.....#########",
        "######################E.....#############",
    ])
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
        "######################E.....#############",
    ])
}