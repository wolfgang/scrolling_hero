use std::io::Cursor;
use std::str;

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

    assert_written_to(&buffer, "#.##\n#.@#\n##.#\n")
}


fn assert_written_to(buffer: &Cursor<Vec<u8>>, written: &str) {
    let reference = buffer.get_ref();
    assert_eq!(written, str::from_utf8(reference).unwrap());
}
