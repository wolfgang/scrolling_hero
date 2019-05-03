use std::io::Cursor;
use std::str;

use console::Key;

use crate::game::Game;

#[test]
fn renders_dungeon_and_player() {
    let dungeon = vec![
        vec![1, 0, 1, 1],
        vec![1, 0, 0, 1],
        vec![1, 1, 0, 1]
    ];

    let game = Game::new(dungeon, (2, 1));

    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();

    assert_lines(&buffer, vec![
        "#.##",
        "#.@#",
        "##.#"
    ]);
}

#[test]
fn player_is_moved_by_cursor_keys() {
    let dungeon = vec![
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 1]
    ];

    let mut game = Game::new(dungeon, (2, 0));

    let mut buffer = Cursor::new(Vec::new());

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#.@#",
        "#..#",
        "#..#"
    ]);

    buffer.set_position(0);

    game.on_key(Key::ArrowDown);

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}

fn assert_lines(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let expected_string = format!("{}\n", expected_lines.join("\n"));
    assert_eq!(str::from_utf8(buffer.get_ref()).unwrap(), expected_string);
}
