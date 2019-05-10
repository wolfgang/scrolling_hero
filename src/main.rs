use console::{Key, Term};

use sch::dungeon_helpers::make_dungeon;
use sch::dungeon_provider::SingleDungeonProvider;
use sch::game::Game;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();

    let (dungeon, player_pos) = make_dungeon(vec![
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
    ]);


    let dungeon_provider = SingleDungeonProvider::shared(dungeon, player_pos);

    let mut game = Game::new(&dungeon_provider, 2);

    loop {
        let num_lines = game.render(&mut term)?;

        let key = term.read_key().unwrap();
        if key == Key::Escape { return Ok(()); }

        game.on_key(key);

        term.clear_last_lines(num_lines as usize)?;
    }
}