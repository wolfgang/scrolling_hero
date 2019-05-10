use std::io::Cursor;
use std::str;

use console::Key;

use crate::dungeon_provider::SingleDungeonProvider;
use crate::game::Game;
use crate::tests::dungeon_helpers::make_dungeon;

#[test]
fn game_can_be_constructed_with_dungeon_provider() {
    let (dungeon, player_pos) = make_dungeon(vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    let provider = SingleDungeonProvider::new(dungeon);

    let game = Game::with_dungeon_provider(&provider, player_pos, 1);

    verify_lines_rendered(&game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}

#[test]
fn player_is_moved_left_right_by_cursor_keys() {
    let mut game = make_game(vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    verify_lines_rendered(&game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered(&game, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered(&game, vec![
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

    verify_lines_rendered(&game, vec![
        "#....",
        "#@..#",
        "#..##"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&game, vec![
        "#...#",
        "#@.##",
        "..###"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&game, vec![
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
    verify_lines_rendered(&game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered(&game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered(&game, vec![
        "#####",
        "###@#",
        "#####"
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_first() {
    let game = make_game(vec![
        "#@...",
        "#...#",
        "#..##",
    ]);

    verify_lines_rendered(&game, vec![
        "#@...",
        "#...#",
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_last() {
    let game = make_game(vec![
        "#....",
        "#...#",
        "#@.##",
    ]);

    verify_lines_rendered(&game, vec![
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
    Game::new(dungeon, player_pos, 1)
}


fn verify_lines_rendered(game: &Game, expected_lines: Vec<&str>) {
    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();
    assert_lines(&buffer, expected_lines);
}

fn assert_lines(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let expected_string = format!("{}\n", expected_lines.join("\n"));
    assert_eq!(str::from_utf8(buffer.get_ref()).unwrap(), expected_string);
}