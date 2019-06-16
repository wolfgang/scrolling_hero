use std::io::Cursor;

use console::Key;

use crate::dungeon::helpers::make_dungeon;
use crate::dungeon::provider::MultiDungeonProvider;
use crate::game::{Game, GameConfig};

use super::game_test_helpers::*;

#[test]
fn is_running_is_false_after_last_dungeon_is_exited() {
    let (dungeon1, player_pos1) = make_dungeon(vec![
        "#.@#",
        "#.E#"
    ]);

    let provider = MultiDungeonProvider::shared(vec![(dungeon1.clone(), player_pos1)]);

    let mut game = Game::with_config(&GameConfig::with_defaults(), &provider);

    game.on_key(Key::ArrowDown);
    assert_eq!(false, game.is_running());
}

#[test]
fn is_running_is_true_until_escape_key() {
    let mut game = make_default_game(vec![
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
    let mut game = make_default_game(vec![
        "#.@#",
        "#.E#"
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#.@#",
        "#.E#"
    ]);
}

#[test]
fn if_player_steps_on_exit_goto_next_dungeon() {
    let mut game = make_game_with_two_dungeons(
        &GameConfig::with_defaults(),
        vec![
            "#..#",
            "#.@#",
            "#.E#"
        ],
        vec![
            "##@#",
            "#E.#",
        ],
    );

    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#.@#",
        "#.E#"
    ]);


    game.on_key(Key::ArrowDown);

    verify_dungeon_rendered(&mut game, vec![
        "##@#",
        "#E.#"
    ]);
}

#[test]
fn player_is_moved_left_right_by_cursor_keys() {
    let mut game = make_default_game(vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}

#[test]
fn player_is_moved_down_by_arrow_down_key_with_scrolling() {
    let mut game = make_default_game(vec![
        "#....",
        "#@..#",
        "#..##",
        "..###",
        "...##",
        "#....#"
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#....",
        "#@..#",
        "#..##"
    ]);

    game.on_key(Key::ArrowDown);
    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#@.##",
        "..###"
    ]);

    game.on_key(Key::ArrowDown);
    verify_dungeon_rendered(&mut game, vec![
        "#..##",
        ".@###",
        "...##"
    ]);
}

#[test]
fn player_collides_with_walls() {
    let mut game = make_default_game(vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_dungeon_rendered(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowRight);
    verify_dungeon_rendered(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);

    game.on_key(Key::ArrowDown);
    verify_dungeon_rendered(&mut game, vec![
        "#####",
        "###@#",
        "#####"
    ]);
}

#[test]
fn player_does_not_move_beyond_last_dungeon_row() {
    let mut game = make_default_game(vec![
        "#...#",
        "#@..#"
    ]);

    game.on_key(Key::ArrowDown);
    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#@..#"
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_first() {
    let mut game = make_default_game(vec![
        "#@...",
        "#...#",
        "#..##",
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#@...",
        "#...#",
    ]);
}

#[test]
fn dont_try_to_render_dungeon_line_beyond_last() {
    let mut game = make_default_game(vec![
        "#....",
        "#...#",
        "#@.##",
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#@.##",
    ]);
}

#[test]
fn render_returns_number_of_lines_rendered() {
    let mut game = make_default_game(vec![
        "#@...",
        "#...#",
        "#..##",
    ]);

    let mut buffer = Cursor::new(Vec::new());
    assert_eq!(game.render(&mut buffer).unwrap(), 2);

    game.on_key(Key::ArrowDown);
    assert_eq!(game.render(&mut buffer).unwrap(), 3);
}