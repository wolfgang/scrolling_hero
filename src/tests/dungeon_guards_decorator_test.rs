use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::tests::dungeon_test_helpers::*;
use crate::types::DungeonLayout;

fn add_guards(dungeon: &mut DungeonLayout, guard_prob: u8, rng: &mut StdRng) {
    for y in 0..dungeon.len() {
        for x in 0..dungeon[y].len() {
            let guard_roll: u8 = rng.gen_range(0, 100);
            if guard_roll < guard_prob && dungeon[y][x] == '.' {
                dungeon[y][x] = 'G';
            }
        }
    }
}


#[test]
fn generates_guards_randomly_on_floor() {
    let dungeon = dungeon_layout(vec![
        "#.....#",
        "###...#",
        "#...#.#",
        "#.#...#",
        "#.....#",
    ]);

    let mut dungeon1 = dungeon.clone();
    let mut dungeon2 = dungeon.clone();
    add_guards(&mut dungeon1, 20, &mut StdRng::seed_from_u64(1000));

    add_guards(&mut dungeon2, 50, &mut StdRng::seed_from_u64(2000));

    assert_eq!(
        dungeon_layout(vec![
            "#.....#",
            "###...#",
            "#...#.#",
            "#.#.G.#",
            "#....G#"
        ]),
        dungeon1);

    assert_eq!(
        dungeon_layout(vec![
            "#...G.#",
            "###.G.#",
            "#.GG#.#",
            "#G#...#",
            "#...G.#"
        ]),
        dungeon2
    );
}
