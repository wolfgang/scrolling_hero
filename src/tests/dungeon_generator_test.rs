use proptest::prelude::*;

use crate::dungeon_generator::generate_dungeon;

#[test]
fn second_line_has_floor_tile_under_entrance() {
    let dungeon = generate_dungeon(16);
    assert_eq!(5, dungeon.len());
    let entrance_x = index_of_first('.', &dungeon[0]);
    assert_eq!('.', char_at(entrance_x, &dungeon[1]));
}

#[test]
fn second_row_contains_consecutive_floor_tiles() {
    let dungeon = generate_dungeon(16);
    let first_floor_tile = index_of_first('.', &dungeon[1]);
    let last_floor_tile = index_of_last('.', &dungeon[1]);
    assert!(last_floor_tile > first_floor_tile, "Expected more than one floor tile");
    let floor_tiles = ".".repeat(last_floor_tile - first_floor_tile + 1);
    assert_eq!(Some(floor_tiles.as_str()), dungeon[1].get(first_floor_tile..last_floor_tile + 1));
}


#[test]
fn remaining_rows_contain_consecutive_floor_tiles() {
    let dungeon = generate_dungeon(16);
    assert_consecutive_floor_tiles(&dungeon[1]);
    assert_consecutive_floor_tiles(&dungeon[2]);
}

fn assert_consecutive_floor_tiles(row: &str) {
    let first_floor_tile = index_of_first('.', row);
    let last_floor_tile = index_of_last('.', row);
    assert!(last_floor_tile > first_floor_tile, "Expected more than one floor tile");
    let floor_tiles = ".".repeat(last_floor_tile - first_floor_tile + 1);
    assert_eq!(Some(floor_tiles.as_str()), row.get(first_floor_tile..last_floor_tile + 1));
}


#[ignore]
#[test]
fn two_dungeons_have_different_number_of_floor_tiles_in_second_row() {
    let dungeon1 = generate_dungeon(16);
    let dungeon2 = generate_dungeon(16);
    assert_ne!(dungeon1[1], dungeon2[1]);
}


proptest! {
    #[test]
    fn first_line_contains_one_entrance(length in 10u32 .. 80) {
        let dungeon = generate_dungeon(length);
        prop_assert!(dungeon[0].contains("#.#"));
        prop_assert_eq!(1, dungeon[0].matches(".").count());
    }

    #[test]
    fn different_dungeons_have_mostly_different_entrance_positions(width in 10u32 .. 80) {
        let dungeon1 = generate_dungeon(width);
        let dungeon2 = generate_dungeon(width);
        let dungeon3 = generate_dungeon(width);
        let dungeon4 = generate_dungeon(width);
        let dungeon5 = generate_dungeon(width);

        let holes = vec![
            index_of_first('.', &dungeon1[0]),
            index_of_first('.', &dungeon2[0]),
            index_of_first('.', &dungeon3[0]),
            index_of_first('.', &dungeon4[0]),
            index_of_first('.', &dungeon5[0]),
            ];


        prop_assert!(has_enough_unique_values(&holes));
    }
}

fn has_enough_unique_values(values: &Vec<usize>) -> bool {
    let mut unique_values = values.clone();
    unique_values.dedup();
    (unique_values.len() as f64 / values.len() as f64) > 0.2
}

fn index_of_first(c: char, s: &str) -> usize {
    let result = s.find(c);
    assert_ne!(None, result);
    result.unwrap()
}

fn index_of_last(c: char, s: &str) -> usize {
    let result = s.rfind(c);
    assert_ne!(None, result);
    result.unwrap()
}

fn char_at(index: usize, s: &str) -> char {
    let chars: Vec<char> = s.chars().collect();
    chars[index]
}

