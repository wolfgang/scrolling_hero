use std::io::{Cursor};
use std::str;

use sch::dungeon_renderer::{render_dungeon, render_dungeon2};

#[test]
fn renders_dungeon() {
    let dungeon = vec![
        "########.##########",
        "###.......#########",
        "######.......######"
    ];

    let mut writer = Cursor::new(Vec::new());
    render_dungeon(&dungeon, &mut writer).unwrap();

    let reference = writer.get_ref();
    assert_eq!(
        "########.##########\n###.......#########\n######.......######\n", 
        str::from_utf8(reference).unwrap());

}


#[test]
fn renders_dungeon_from_vectors() {
    let dungeon = vec![
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 0, 1, 1]
    ];

    assert_eq!(1, dungeon[1][1]);
    assert_eq!(0, dungeon[2][4]);

    let mut writer = Cursor::new(Vec::new());
    render_dungeon2(&dungeon, &mut writer).unwrap();

    let reference = writer.get_ref();
    assert_eq!(
        "####.####\n##....###\n####.#.##\n", 
        str::from_utf8(reference).unwrap());
}
