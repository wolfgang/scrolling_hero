use std::io::Cursor;
use std::str;

use regex::Regex;

use crate::dungeon::helpers::make_dungeon;
use crate::dungeon::provider::SingleDungeonProvider;
use crate::game::{Game, GameConfig};

type LineBuffer = Cursor<Vec<u8>>;
type Lines<'a> = Vec<&'a str>;

pub fn make_default_game(dungeon_definition: Vec<&str>) -> Game {
    make_game_with_config(&GameConfig::with_defaults(), dungeon_definition)
}

pub fn make_game_with_config(config: &GameConfig, dungeon_definition: Vec<&str>) -> Game {
    let (dungeon, player_pos) = make_dungeon(dungeon_definition);
    Game::with_config(
        config,
        &SingleDungeonProvider::shared(dungeon, player_pos))
}

pub fn verify_dungeon_rendered(game: &mut Game, expected_lines: Lines) {
    let buffer = render(game);
    assert_lines_start_with(&buffer, expected_lines);
}

pub fn verify_lines_rendered_match(game: &mut Game, expected_lines: Lines) {
    let buffer = render(game);
    assert_lines_match(&buffer, expected_lines);
}

pub fn render(game: &mut Game) -> LineBuffer {
    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();
    buffer
}

pub fn assert_lines_start_with(buffer: &LineBuffer, expected_lines: Lines) {
    let rendered_lines = lines_from(buffer);
    assert_eq!(rendered_lines.len(), expected_lines.len());

    let line_length = expected_lines[0].len();
    let actual_lines: Vec<String> = rendered_lines[0..expected_lines.len()]
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(actual_lines, expected_lines);
}

pub fn assert_lines_match(buffer: &LineBuffer, expected_lines: Lines) {
    let actual_lines = lines_from(buffer);

    for (i, line) in expected_lines.iter().enumerate() {
        let re = Regex::new(line).unwrap();
        assert!(
            re.is_match(actual_lines[i]),
            format!("Expected {} to match {}", actual_lines[i], line));
    }
}

fn lines_from(buffer: &LineBuffer) -> Lines {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    let lines: Vec<&str> = actual_string.split("\n").collect();
    lines[0..lines.len() - 1].to_vec()
}