use std::io::{Cursor, Write};
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


#[test]
fn renders_dungeon_from_vectors() {
    let dungeon = vec![
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 0, 1, 1]
    ];

    let mut writer = Cursor::new(Vec::new());
    render_dungeon2(&dungeon, &mut writer).unwrap();

    let reference = writer.get_ref();
    assert_eq!(
        "####.####\n##....###\n####.#.##\n", 
        str::from_utf8(reference).unwrap());
}

fn render_dungeon2(dungeon: &Vec<Vec<u16>>, writer: &mut Write) -> std::io::Result<()> {
    for row in dungeon {
        let mut row_str = String::new();
        for cell in row {
            if *cell == 0 { row_str.push('.')}
            if *cell == 1 { row_str.push('#')}
        }

        writer.write_fmt(format_args!("{}\n", &row_str))?;
    }

    Ok(())
}