use proptest::prelude::*;
use rand::{Rng, thread_rng};

fn generate_dungeon(length: u32) -> Vec<String> {
    let mut rng = thread_rng();

    let entrance_x = rng.gen_range(1, length - 1);

    let mut line1 = String::with_capacity(length as usize);
    for i in 0..length {
        if i == entrance_x { line1.push('.') } else { line1.push('#') }
    }

    let mut line2 = line1.clone();
    line2.replace_range(entrance_x as usize..entrance_x as usize + 1, "..");


    vec![line1, line2]
}

#[test]
fn how_to_detect_consecutive_floor_tiles() {
    let str1 = String::from("###...####");

    let first = str1.find('.').unwrap();
    let last = str1.rfind('.').unwrap();
    assert_eq!(3, first);
    assert_eq!(5, last);
    assert_eq!(Some("..."), str1.get(first..last + 1));

    assert_eq!(Some(".".repeat(last - first + 1).as_str()), str1.get(first..last + 1));
}

#[test]
fn second_line_contains_consecutive_floor_tiles_under_entrance() {
    let dungeon = generate_dungeon(16);
    assert_eq!(2, dungeon.len());
    let entrance_x = index_of_first('.', &dungeon[0]);
    assert_eq!('.', char_at(entrance_x, &dungeon[1]));

    let first_floor_tile = index_of_first('.', &dungeon[1]);
    let last_floor_tile = index_of_last('.', &dungeon[1]);
    assert!(last_floor_tile > first_floor_tile, "Expected more than one floor tile");
    let floor_tiles = ".".repeat(last_floor_tile - first_floor_tile + 1);
    assert_eq!(Some(floor_tiles.as_str()), dungeon[1].get(first_floor_tile..last_floor_tile + 1));

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

