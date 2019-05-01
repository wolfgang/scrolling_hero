use std::io::Cursor;
use std::str;

use sch::dungeon_renderer::DungeonRenderer;
use sch::player::Player0;

#[test]
fn renders_dungeon_from_vectors() {
    let dungeon = vec![
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 0, 1, 1]
    ];

    let player = Player0::new(4, 1, 100, 100);
    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player);
    let mut buffer = Cursor::new(Vec::new());
    player.move_right();

    let num_lines = dungeon_renderer.render(&mut buffer, 1, 2).unwrap();
    assert_eq!(2, num_lines);
    assert_written_to(&buffer, "##...@###\n####.#.##\n");
}

#[test]
fn renders_dungeon_not_beyond_end() {
    let dungeon = vec![
        vec![1, 1, 1],
        vec![1, 0, 0],
        vec![1, 0, 1],
        vec![0, 0, 1]
    ];

    let player = Player0::new(0, 0, 100, 100);

    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player);
    let mut buffer = Cursor::new(Vec::new());

    let num_lines = dungeon_renderer.render(&mut buffer, 2, 5).unwrap();
    assert_eq!(2, num_lines);
    assert_written_to(&buffer, "#.#\n..#\n");
}

#[test]
fn renders_dungeon_not_beyond_beginning() {
    let dungeon = vec![
        vec![1, 1, 1],
        vec![1, 0, 0],
        vec![1, 0, 1],
        vec![0, 0, 1]
    ];

    let player = Player0::new(3, 3, 100, 100);
    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player);
    let mut buffer = Cursor::new(Vec::new());

    let num_lines = dungeon_renderer.render(&mut buffer, -1, 3).unwrap();
    assert_eq!(4, num_lines);
    assert_written_to(&buffer, "###\n#..\n#.#\n..#\n");
}


fn assert_written_to(buffer: &Cursor<Vec<u8>>, written: &str) {
    let reference = buffer.get_ref();
    assert_eq!(written, str::from_utf8(reference).unwrap());
}
