use crate::dungeon_helpers::make_dungeon;
use crate::types::DungeonLayout;

#[allow(dead_code)]
pub fn print_dungeon_snapshot(dungeon: &DungeonLayout) {
    for row in dungeon {
        println!("\"{}\",", row.into_iter().collect::<String>());
    }
}

pub fn dungeon_layout(strings: Vec<&str>) -> DungeonLayout {
    let dungeon = make_dungeon(strings);
    dungeon.0
}
