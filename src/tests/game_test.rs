use std::io::Cursor;
use std::str;

use console::Key;

use crate::game::Game;

#[test]
fn player_is_moved_left_right_by_cursor_keys() {
    let dungeon = make_dungeon(vec![
        "#..#",
        "#..#",
        "#..#"
    ]);
    let (mut game, mut buffer) = setup(dungeon, (2, 1));

    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}

#[test]
fn player_is_moved_down_by_arrow_down_key_with_scrolling() {
    let dungeon = make_dungeon(vec![
        "#....",
        "#...#",
        "#..##",
        "..###",
        "...##",
        "#....#"
    ]);

    let (mut game, mut buffer) = setup(dungeon, (1, 1));

    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#....",
        "#@..#",
        "#..##"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#...#",
        "#@.##",
        "..###"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#..##",
        ".@###",
        "...##"
    ]);
}

#[test]
fn player_collides_with_walls() {
    let dungeon = make_dungeon(vec![
        "#####",
        "###.#",
        "#####",
    ]);
    let (mut game, mut buffer) = setup(dungeon, (3, 1));

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#####",
        "###@#",
        "#####"
    ]);


    game.on_key(Key::ArrowRight);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#####",
        "###@#",
        "#####"
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_first() {
    let dungeon = make_dungeon(vec![
        "#....",
        "#...#",
        "#..##",
    ]);

    let (mut game, mut buffer) = setup(dungeon, (1, 0));

    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#@...",
        "#...#",
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_last() {
    let dungeon = make_dungeon(vec![
        "#....",
        "#...#",
        "#..##",
    ]);

    let (mut game, mut buffer) = setup(dungeon, (1, 2));

    verify_lines_rendered(&mut game, &mut buffer, vec![
        "#...#",
        "#@.##",
    ]);
}

#[test]
fn render_returns_number_of_lines_rendered() {
    let dungeon = make_dungeon(vec![
        "#....",
        "#...#",
        "#..##",
    ]);

    let (mut game, mut buffer) = setup(dungeon, (1, 0));

    assert_eq!(game.render(&mut buffer).unwrap(), 2);
    game.on_key(Key::ArrowDown);
    assert_eq!(game.render(&mut buffer).unwrap(), 3);
}

fn make_dungeon(strings: Vec<&str>) -> Vec<Vec<u16>> {
    let mut result = Vec::new();

    for row in strings.iter() {
        let mut result_row = Vec::new();
        for c in (*row).chars() {
            if c == '.' { result_row.push(0) }
            if c == '#' { result_row.push(1) }
        }
        result.push(result_row);
    }
    result
}


fn setup(dungeon: Vec<Vec<u16>>, player_position: (u32, u32)) -> (Game, Cursor<Vec<u8>>) {
    let game = Game::new(dungeon, player_position);
    let buffer = Cursor::new(Vec::new());
    (game, buffer)
}

fn verify_lines_rendered(game: &mut Game, mut buffer: &mut Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    buffer.set_position(0);
    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, expected_lines);
}

fn assert_lines(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let expected_string = format!("{}\n", expected_lines.join("\n"));
    assert_eq!(str::from_utf8(buffer.get_ref()).unwrap(), expected_string);
}
