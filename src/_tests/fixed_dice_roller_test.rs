use std::collections::HashMap;

trait DiceRoller {
    fn roll(&mut self, dice: u8) -> u8;
}

struct FixedDiceRoller {
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

    pub fn next_roll(&mut self, dice: u8, value: u8) {
        let next_rolls_for_dice = self.next_rolls.entry(dice).or_insert(Vec::with_capacity(10));
        next_rolls_for_dice.push(value);
    }
}

impl DiceRoller for FixedDiceRoller {
    fn roll(&mut self, dice: u8) -> u8 {
        let index_for_dice = self.index.entry(dice).or_insert(0);
        let roll = self.next_rolls.get(&dice).unwrap()[*index_for_dice];
        *index_for_dice += 1;
        roll
    }
}


#[test]
fn roll_a_fixed_sequence_of_d20s() {
    let mut roller = FixedDiceRoller::new();
    roller.next_roll(20, 10);
    roller.next_roll(20, 15);
    roller.next_roll(20, 1);

    assert_eq!(roller.roll(20), 10);
    assert_eq!(roller.roll(20), 15);
    assert_eq!(roller.roll(20), 1);
}

#[test]
fn roll_a_fixed_sequence_of_various_dice() {
    let mut roller = FixedDiceRoller::new();
    roller.next_roll(20, 10);
    roller.next_roll(10, 3);
    roller.next_roll(10, 7);
    roller.next_roll(20, 15);

    assert_eq!(roller.roll(20), 10);
    assert_eq!(roller.roll(20), 15);
    assert_eq!(roller.roll(10), 3);
    assert_eq!(roller.roll(10), 7);
}