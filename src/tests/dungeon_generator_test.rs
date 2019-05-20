use crate::types::DungeonLayout;

fn generate_dungeon(width: u32) -> DungeonLayout {
    let mut first_row = Vec::with_capacity(width as usize);
    first_row.push('#');
    for _i in 0..width - 2 {
        first_row.push('.');
    }
    first_row.push('#');


    vec![first_row]
}

#[test]
fn first_row_is_all_floor_except_left_right_walls() {
    let dungeon1 = generate_dungeon(10);
    assert_eq!(as_chars("#........#"), dungeon1[0]);

    let dungeon2 = generate_dungeon(5);
    assert_eq!(as_chars("#...#"), dungeon2[0]);
}


fn as_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}