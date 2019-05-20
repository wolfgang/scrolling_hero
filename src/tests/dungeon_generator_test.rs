use rand::{Rng, thread_rng};

use crate::dungeon_helpers::make_dungeon;
use crate::types::DungeonLayout;

fn generate_dungeon_init(width: usize, height: usize) -> DungeonLayout {
    let mut open_row = vec!['.'; width as usize];
    open_row[0] = '#';
    open_row[width as usize - 1] = '#';

    let mut dungeon = vec![vec!['#'; width]; height as usize];
    dungeon[0] = open_row.to_vec();
    dungeon[height - 1] = open_row.to_vec();
    dungeon
}

fn generate_dungeon_step(dungeon: &mut DungeonLayout) {
    // Choose floor tile in first row
    // Punch floor tile below that
    // Choose random direction (left, down, right)
    // Punch floor tile there
    // Choose random direction except the one we came from
    // Punch floor tile there
    // Repeat until we hit an existing floor tile or last row

    let width = dungeon[0].len();
    let height = dungeon.len();

    let mut rng = thread_rng();

    let x = rng.gen_range(2, width - 2);
    dungeon[1][x] = '.';

    let direction = rng.gen_range(0, 3);

    match direction {
        0 => {
            // Left
            dungeon[1][x - 1] = '.';
        }
        1 => {
            // Down
            dungeon[2][x] = '.';
        }
        2 => {
            // Right
            dungeon[1][x + 1] = '.';
        }

        _ => {}
    }
}

#[test]
fn first_iteration_is_all_walls_except_first_and_last_line() {
    let dungeon = generate_dungeon_init(5, 4);

    assert_eq!(
        dungeon_from(vec![
            "#...#",
            "#####",
            "#####",
            "#...#",
        ]),
        dungeon)
}

#[test]
fn second_iteration_carves_a_path() {
    let mut dungeon = generate_dungeon_init(5, 4);

    // Do this with a fixed seed and use first result that looks right as ref
    generate_dungeon_step(&mut dungeon);
    assert_eq!(
        dungeon_from(vec![
            "#...#",
            "#..##",
            "##.##",
            "#...#",
        ]),
        dungeon)
}


fn as_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn dungeon_from(strings: Vec<&str>) -> DungeonLayout {
    let dungeon = make_dungeon(strings);
    dungeon.0
}