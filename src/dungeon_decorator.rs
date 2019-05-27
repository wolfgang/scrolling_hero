use rand::Rng;
use rand::rngs::StdRng;

use crate::types::DungeonLayout;

pub fn add_guards(dungeon: &mut DungeonLayout, guard_prob: u8, rng: &mut StdRng) {
    for y in 0..dungeon.len() {
        for x in 0..dungeon[y].len() {
            let guard_roll: u8 = rng.gen_range(0, 100);
            if guard_roll < guard_prob && dungeon[y][x] == '.' {
                dungeon[y][x] = 'G';
            }
        }
    }
}
