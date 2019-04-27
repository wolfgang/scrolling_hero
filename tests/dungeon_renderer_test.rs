use std::io::{Cursor, Write};
use std::str;

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

fn render_dungeon(dungeon: &Vec<&str>, writer: &mut Write) -> std::io::Result<()> {
    for line in dungeon {
        writer.write_fmt(format_args!("{}\n", line))?;
    }

    Ok(())
}