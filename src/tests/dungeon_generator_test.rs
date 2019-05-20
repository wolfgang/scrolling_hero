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

fn generate_dungeon_step(width: usize, height: usize) -> DungeonLayout {
    let mut open_row = vec!['.'; width as usize];
    open_row[0] = '#';
    open_row[width as usize - 1] = '#';

    let mut dungeon = vec![vec!['#'; width]; height as usize];
    dungeon[0] = open_row.to_vec();
    dungeon[height - 1] = open_row.to_vec();
    dungeon
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