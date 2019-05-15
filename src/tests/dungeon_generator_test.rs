use proptest::prelude::*;
use rand::{Rng, thread_rng};

fn generate_dungeon(length: u32) -> Vec<String> {
    let mut rng = thread_rng();

    let entrance_x = rng.gen_range(1, length - 1);

    let mut line1 = String::with_capacity(length as usize);
    for i in 0..length {
        if i == entrance_x { line1.push('.') } else { line1.push('#') }
    }

    let line2 = line1.clone();

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
    let entrance_x = dungeon[0].find('.').unwrap();
    assert_eq!(Some(entrance_x), dungeon[1].find('.'));
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
        let hole1 = dungeon1[0].find('.').unwrap();

        let holes = vec![
            dungeon1[0].find('.').unwrap(),
            dungeon2[0].find('.').unwrap(),
            dungeon3[0].find('.').unwrap(),
            dungeon4[0].find('.').unwrap(),
            dungeon5[0].find('.').unwrap()
            ];
        let iter = holes.into_iter().filter(|x| *x != hole1);
        prop_assert!(iter.count() > 0);
    }
}

