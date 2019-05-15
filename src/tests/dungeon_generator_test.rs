use proptest::prelude::*;
use rand::{Rng, thread_rng};

fn generate_dungeon(length: u32) -> Vec<String> {
    let mut rng = thread_rng();

    let hole_position = rng.gen_range(1, length - 1);

    let mut result = String::with_capacity(length as usize);
    for i in 0..length {
        if i == hole_position { result.push('.') } else { result.push('#') }
    }

    vec![result]
}

proptest! {
    #[test]
    fn first_line_contains_one_hole(length in 10u32 .. 80) {
        let dungeon = generate_dungeon(length);
        prop_assert!(dungeon[0].contains("#.#"));
        prop_assert_eq!(1, dungeon[0].matches(".").count());
    }

    #[test]
    fn different_dungeons_have_mostly_different_entrance_hole_positions(width in 10u32 .. 80) {
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

