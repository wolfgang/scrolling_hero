use std::cmp::{max, min};

use proptest::prelude::*;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

fn generate_dungeon(length: u32) -> Vec<String> {
    let mut rng = thread_rng();

    let entrance_x = rng.gen_range(1, length - 1);

    let mut line1 = String::with_capacity(length as usize);
    for i in 0..length {
        if i == entrance_x { line1.push('.') } else { line1.push('#') }
    }

    let line2 = generate_row(&line1, entrance_x, &mut rng);
    let line3 = generate_row(&line1, entrance_x, &mut rng);
    let line4 = generate_row(&line1, entrance_x, &mut rng);
    let line5 = generate_row(&line1, entrance_x, &mut rng);

    vec![line1, line2, line3, line4, line5]
}

fn generate_row(first_row: &String, entrance_x: u32, rng: &mut ThreadRng) -> String {
    let mut row = first_row.clone();

    let offset1 = rng.gen_range(0, 5);
    let offset2 = rng.gen_range(0, 5);

    let from = max(1, entrance_x as i32 - offset1) as usize;
    let to = min(first_row.len() as u32 - 1, entrance_x + offset2) as usize;

    row.replace_range(from..to, ".".repeat(to - from + 1).as_str());
    row
}

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
        let hole1 = index_of_first('.', &dungeon1[0]);

        let holes = vec![
            index_of_first('.', &dungeon1[0]),
            index_of_first('.', &dungeon2[0]),
            index_of_first('.', &dungeon3[0]),
            index_of_first('.', &dungeon4[0]),
            index_of_first('.', &dungeon5[0]),
            ];
        let positions_different_from_first = holes.into_iter().filter(|x| *x != hole1);
        prop_assert!(positions_different_from_first.count() > 0);
    }
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

