use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

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

const LEFT: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;

fn generate_dungeon_path(dungeon: &mut DungeonLayout, rng_seed: u64) {
    let width = dungeon[0].len();
    let height = dungeon.len();

    let mut rng = StdRng::seed_from_u64(rng_seed);

    let mut x = rng.gen_range(1, width - 2);
    let mut y = 1;
    dungeon[1][x] = '.';

    let mut prev_x = x;

    while y < height - 1 {
        let mut directions = Vec::new();
        directions.push(DOWN);
        if x - 1 > 1 && x - 1 != prev_x {
            directions.push(LEFT);
            directions.push(LEFT);
        };
        if x + 1 < width - 1 && x + 1 != prev_x {
            directions.push(RIGHT);
            directions.push(RIGHT);
        };

        prev_x = x;

        let index = rng.gen_range(0, directions.len());
        let direction = directions[index];

        match direction {
            LEFT => {
                x = x - 1;
            }
            DOWN => {
                y = y + 1;
            }
            RIGHT => {
                x = x + 1;
            }

            _ => {}
        }

        dungeon[y][x] = '.';
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
    let mut dungeon = generate_dungeon_init(10, 8);

    // Do this with a fixed seed and use first result that looks right as ref
    generate_dungeon_path(&mut dungeon, 1000);

    assert_eq!(
        dungeon_from(vec![
            "#........#",
            "#######..#",
            "####.....#",
            "####...###",
            "######...#",
            "#######..#",
            "#######..#",
            "#........#"]),
        dungeon
    );
}


//fn as_chars(s: &str) -> Vec<char> {
//    s.chars().collect()
//}
//

fn dungeon_from(strings: Vec<&str>) -> DungeonLayout {
    let dungeon = make_dungeon(strings);
    dungeon.0
}