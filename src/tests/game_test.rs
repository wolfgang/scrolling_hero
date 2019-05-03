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

    let mut game = Game::new(dungeon, (2, 1));

    let mut buffer = Cursor::new(Vec::new());

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    buffer.set_position(0);
    game.on_key(Key::ArrowLeft);

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    buffer.set_position(0);
    game.on_key(Key::ArrowRight);

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
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


    let mut game = Game::new(dungeon, (1, 1));
    let mut buffer = Cursor::new(Vec::new());

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#....",
        "#@..#",
        "#..##"
    ]);

    buffer.set_position(0);
    game.on_key(Key::ArrowDown);

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#...#",
        "#@.##",
        "..###"
    ]);

    buffer.set_position(0);
    game.on_key(Key::ArrowDown);

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#..##",
        ".@###",
        "...##"
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_first() {
    let dungeon = make_dungeon(vec![
        "#....",
        "#...#",
        "#..##",
    ]);

    let mut game = Game::new(dungeon, (1, 0));
    let mut buffer = Cursor::new(Vec::new());

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
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

    let mut game = Game::new(dungeon, (1, 2));
    let mut buffer = Cursor::new(Vec::new());

    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, vec![
        "#...#",
        "#@.##",
    ]);
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

fn assert_lines(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let expected_string = format!("{}\n", expected_lines.join("\n"));
    assert_eq!(str::from_utf8(buffer.get_ref()).unwrap(), expected_string);
}
