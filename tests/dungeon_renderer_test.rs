use std::io::{Cursor};
use std::str;
use std::rc::Rc;
use std::cell::{RefCell};

use sch::dungeon_renderer::{DungeonRenderer};

#[test]
fn renders_dungeon_from_vectors() {
    let dungeon = vec![
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 0, 1, 1]
    ];


    let mut writer = Cursor::new(Vec::new());

    let  player_pos = Rc::new(RefCell::new((4, 1)));

    let dungeon_renderer = DungeonRenderer::new(&dungeon, &player_pos);

    dungeon_renderer.render(&mut writer).unwrap();

    let reference = writer.get_ref();
    assert_eq!(
        "####.####\n##..@.###\n####.#.##\n", 
        str::from_utf8(reference).unwrap());

    // player_pos.0 = 5;

    // writer.set_position(0);

    // dungeon_renderer.render(&mut writer).unwrap();

    // let reference2 = writer.get_ref();
    // assert_eq!(
    //     "####.####\n##...@###\n####.#.##\n", 
    //     str::from_utf8(reference2).unwrap());

}
