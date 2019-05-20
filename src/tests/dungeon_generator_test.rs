use crate::dungeon_helpers::make_dungeon;
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

fn generate_dungeon_step(width: u32, height: u32) -> DungeonLayout {
    let mut rows = Vec::with_capacity(height as usize);

    let mut first_row = Vec::with_capacity(width as usize);
    first_row.push('#');
    for _i in 0..width - 2 {
        first_row.push('.');
    }
    first_row.push('#');

    rows.push(first_row.to_vec());
    for _i in 0..height - 2 {
        rows.push(vec!['#'; width as usize]);
    }

    rows.push(first_row.to_vec());

    rows
}

#[test]
fn first_row_is_all_floor_except_left_right_walls() {
    let dungeon1 = generate_dungeon(10);
    assert_eq!(as_chars("#........#"), dungeon1[0]);

    let dungeon2 = generate_dungeon(5);
    assert_eq!(as_chars("#...#"), dungeon2[0]);
}

#[test]
fn first_iteration_is_all_walls_except_first_and_last_line() {
    let dungeon = generate_dungeon_step(5, 4);

    assert_eq!(
        make_dungeon(vec![
            "#...#",
            "#####",
            "#####",
            "#...#",
        ]),
        (dungeon, (0, 0)))
}

fn as_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}