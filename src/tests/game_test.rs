use std::io::Cursor;
use std::str;

use console::Key;

use crate::dungeon_helpers::make_dungeon;
use crate::dungeon_provider::{MultiDungeonProvider, SingleDungeonProvider};
use crate::game::Game;

#[test]
fn prints_number_of_turns() {
    let mut game = make_game(vec![
        "#.@#",
        "#..#"
    ]);

    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    let lines: Vec<&str> = actual_string.split("\n").collect();

    assert_ne!(None, lines[0].find("Steps: 0"));
}

#[test]
fn is_running_is_false_after_last_dungeon_is_exited() {
    let (dungeon1, player_pos1) = make_dungeon(vec![
        "#.@#",
        "#.E#"
    ]);

    let provider = MultiDungeonProvider::shared(vec![(dungeon1.clone(), player_pos1)]);

    let mut game = Game::new(&provider, 1);

    game.on_key(Key::ArrowDown);
    assert_eq!(false, game.is_running());
}

#[test]
fn is_running_is_true_until_escape_key() {
    let mut game = make_game(vec![
        "#.@#",
        "#..#"
    ]);

    assert_eq!(true, game.is_running());

    game.on_key(Key::ArrowDown);
    game.on_key(Key::ArrowLeft);
    game.on_key(Key::ArrowRight);
    assert_eq!(true, game.is_running());

    game.on_key(Key::Escape);
    assert_eq!(false, game.is_running());
}

#[test]
fn renders_exit() {
    let mut game = make_game(vec![
        "#.@#",
        "#.E#"
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#.@#",
        "#.E#"
    ]);
}

#[test]
fn if_player_steps_on_exit_goto_next_dungeon() {
    let (dungeon1, player_pos1) = make_dungeon(vec![
        "#..#",
        "#.@#",
        "#.E#"
    ]);

    let (dungeon2, player_pos2) = make_dungeon(vec![
        "##@#",
        "#E.#",
    ]);

    let provider = MultiDungeonProvider::shared(vec![
        (dungeon1.clone(), player_pos1),
        (dungeon2.clone(), player_pos2),
    ]);

    let mut game = Game::new(&provider, 1);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#.@#",
        "#.E#"
    ]);

    game.on_key(Key::ArrowDown);

    verify_lines_rendered_start_with(&mut game, vec![
        "##@#",
        "#E.#"
    ]);

}

#[test]
fn player_is_moved_left_right_by_cursor_keys() {
    let mut game = make_game(vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}

#[test]
fn player_is_moved_down_by_arrow_down_key_with_scrolling() {
    let mut game = make_game(vec![
        "#....",
        "#@..#",
        "#..##",
        "..###",
        "...##",
        "#....#"
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#....",
        "#@..#",
        "#..##"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#@.##",
        "..###"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..##",
        ".@###",
        "...##"
    ]);
}

#[test]
fn player_collides_with_walls() {
    let mut game = make_game(vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered_start_with(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_first() {
    let mut game = make_game(vec![
        "#@...",
        "#...#",
        "#..##",
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#@...",
        "#...#",
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_last() {
    let mut game = make_game(vec![
        "#....",
        "#...#",
        "#@.##",
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#@.##",
    ]);
}

#[test]
fn render_returns_number_of_lines_rendered() {
    let mut game = make_game(vec![
        "#@...",
        "#...#",
        "#..##",
    ]);

    let mut buffer = Cursor::new(Vec::new());
    assert_eq!(game.render(&mut buffer).unwrap(), 2);

    game.on_key(Key::ArrowDown);
    assert_eq!(game.render(&mut buffer).unwrap(), 3);
}

fn make_game(strings: Vec<&str>) -> Game {
    let (dungeon, player_pos) = make_dungeon(strings);
    Game::new(&SingleDungeonProvider::shared(dungeon, player_pos), 1)
}


fn verify_lines_rendered_start_with(game: &mut Game, expected_lines: Vec<&str>) {
    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();
    assert_lines_start_with(&buffer, expected_lines);
}


fn assert_lines_start_with(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    let lines: Vec<&str> = actual_string.split("\n").collect();

    let line_length = expected_lines[0].len();
    let actual_lines: Vec<String> = lines[0..expected_lines.len()]
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(expected_lines, actual_lines);
}