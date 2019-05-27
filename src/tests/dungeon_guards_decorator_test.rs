use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::tests::dungeon_test_helpers::*;
use crate::types::DungeonLayout;

fn add_guards(dungeon: &mut DungeonLayout, rng: &mut StdRng) {
    for y in 0..dungeon.len() {
        for x in 0..dungeon[y].len() {
            let guard_roll = rng.gen_range(0, 100);
            if guard_roll < 20 && dungeon[y][x] == '.' {
                dungeon[y][x] = 'G';
            }
        }
    }
}


#[test]
fn generates_guards_randomly_on_floor() {
    let mut dungeon = dungeon_layout(vec![
        "#.....#",
        "###...#",
        "#...#.#",
        "#.#...#",
        "#.....#",
    ]);

    let mut rng = StdRng::seed_from_u64(1000);

    add_guards(&mut dungeon, &mut rng);

    print_dungeon_snapshot(&dungeon);
}
