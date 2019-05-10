use crate::types::DungeonDefinition;

pub fn make_dungeon(strings: Vec<&str>) -> DungeonDefinition {
    let mut result = Vec::new();
    let mut player_pos = (0, 0);

    for (y, row) in strings.iter().enumerate() {
        let mut result_row = Vec::new();
        for (x, c) in (*row).chars().enumerate() {
            if c == '@' {
                player_pos = (x as u32, y as u32);
                result_row.push(0);
            }
            if c == '.' { result_row.push(0) }
            if c == '#' { result_row.push(1) }
        }
        result.push(result_row);
    }
    (result, player_pos)
}