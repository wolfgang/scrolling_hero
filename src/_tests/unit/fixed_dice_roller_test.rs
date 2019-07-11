use crate::game::dice_roller::DiceRoller;

use super::fixed_dice_roller::FixedDiceRoller;

struct DiceUser {
    pub roll: u8
}

impl DiceUser {
    pub fn use_roll(&mut self, roller: &mut DiceRoller) {
        self.roll = roller.roll(20)
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

#[test]
fn can_pass_trait_ref_to_function() {
    let mut user = DiceUser { roll: 0 };
    let mut roller = FixedDiceRoller::new();
    roller.next_roll(20, 15);
    user.use_roll(&mut roller);
    assert_eq!(user.roll, 15);
}

#[test]
fn if_no_roll_configured_return_1() {
    let mut roller = FixedDiceRoller::new();
    assert_eq!(roller.roll(10), 1);
    assert_eq!(roller.roll(20), 1);
}