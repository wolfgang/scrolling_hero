use std::io::{Cursor};
use std::str;

use sch::dungeon_renderer::render_dungeon;

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
