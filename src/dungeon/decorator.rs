use rand::Rng;
use rand::rngs::StdRng;

use crate::types::DungeonLayout;

pub fn add_guards(dungeon: &mut DungeonLayout, guard_prob: u8, rng: &mut StdRng) {
    add_dungeon_objects(dungeon, guard_prob, 'G', rng);
}


fn add_dungeon_objects(dungeon: &mut DungeonLayout, object_prob: u8, object_char: char, rng: &mut StdRng) {
    for y in 1..dungeon.len() - 1 {
        for x in 0..dungeon[y].len() {
            let guard_roll: u8 = rng.gen_range(0, 100);
            if guard_roll < object_prob && dungeon[y][x] == '.' {
                dungeon[y][x] = object_char;
            }
        }
    }
}
