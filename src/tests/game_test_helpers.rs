use std::io::Cursor;
use std::str;

use regex::Regex;

use crate::dungeon_helpers::make_dungeon;
use crate::dungeon_provider::SingleDungeonProvider;
use crate::game::Game;

pub fn make_game(strings: Vec<&str>) -> Game {
    let (dungeon, player_pos) = make_dungeon(strings);
    Game::new(&SingleDungeonProvider::shared(dungeon, player_pos), 1)
}


pub fn verify_lines_rendered_start_with(game: &mut Game, expected_lines: Vec<&str>) {
    let buffer = render(game);
    assert_lines_start_with(&buffer, expected_lines);
}

pub fn verify_lines_rendered_match(game: &mut Game, expected_lines: Vec<&str>) {
    let buffer = render(game);
    assert_lines_match(&buffer, expected_lines);
}

pub fn render(game: &mut Game) -> Cursor<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    game.render(&mut buffer).unwrap();
    buffer
}

pub fn assert_lines_start_with(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let rendered_lines = lines_from(buffer);

    let line_length = expected_lines[0].len();
    let actual_lines: Vec<String> = rendered_lines[0..expected_lines.len()]
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(expected_lines, actual_lines);
}

pub fn assert_lines_match(buffer: &Cursor<Vec<u8>>, expected_lines: Vec<&str>) {
    let actual_lines = lines_from(buffer);

    for (i, line) in expected_lines.iter().enumerate() {
        let re = Regex::new(line).unwrap();
        assert!(
            re.is_match(actual_lines[i]),
            format!("Expected {} to match {}", actual_lines[i], line));
    }
}

pub fn lines_from(buffer: &Cursor<Vec<u8>>) -> Vec<&str> {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    actual_string.split("\n").collect()
}