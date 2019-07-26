use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::game::dice_roller::DiceRoller;

pub struct FixedDiceRoller {
    next_rolls: HashMap<u8, Vec<u8>>,
    index: HashMap<u8, usize>,
}

impl FixedDiceRoller {
    pub fn new() -> FixedDiceRoller {
        FixedDiceRoller {
            next_rolls: HashMap::new(),
            index: HashMap::new(),
        }
    }

    pub fn shared() -> Rc<RefCell<FixedDiceRoller>> {
        Rc::new(RefCell::new(FixedDiceRoller::new()))
    }

    pub fn next_roll(&mut self, dice: u8, value: u8) {
        let next_rolls_for_dice = self.next_rolls.entry(dice).or_insert(Vec::with_capacity(10));
        next_rolls_for_dice.push(value);
    }
}

impl DiceRoller for FixedDiceRoller {
    fn roll(&mut self, dice: u8) -> u8 {
        let index_for_dice = self.index.entry(dice).or_insert(0);
        if !self.next_rolls.contains_key(&dice) { return 1; };
        let roll = self.next_rolls.get(&dice).unwrap()[*index_for_dice];
        *index_for_dice += 1;
        roll
    }
}
