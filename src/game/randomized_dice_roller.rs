use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use super::dice_roller::DiceRoller;

pub struct RandomizedDiceRoller {
    rng: StdRng
}

impl RandomizedDiceRoller {
    pub fn new() -> RandomizedDiceRoller {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        RandomizedDiceRoller {
            rng: StdRng::seed_from_u64(seed)
        }
    }
}

impl DiceRoller for RandomizedDiceRoller {
    fn roll(&mut self, dice: u8) -> u8 {
        self.rng.gen_range(1, dice + 1)
    }
}
