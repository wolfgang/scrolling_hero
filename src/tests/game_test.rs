use std::io::Cursor;
use std::str;

use console::Key;

use crate::dungeon_helpers::make_dungeon;
use crate::dungeon_provider::{MultiDungeonProvider, SingleDungeonProvider};
use crate::game::Game;

#[test]
fn renders_exit() {
    let game = make_game(vec![
        "#.@#",
        "#.E#"
    ]);

    verify_lines_rendered(&game, vec![
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
    verify_lines_rendered(&game, vec![
        "#..#",
        "#.@#",
        "#.E#"
    ]);

    game.on_key(Key::ArrowDown);

    verify_lines_rendered(&game, vec![
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
    Game::new(&SingleDungeonProvider::shared(dungeon, player_pos), 1)
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