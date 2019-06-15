struct FixedDiceRoller {
    next_rolls: Vec<u8>,
    index: usize,
}

impl FixedDiceRoller {
    pub fn new() -> FixedDiceRoller {
        FixedDiceRoller {
            next_rolls: Vec::with_capacity(10),
            index: 0,
        }
    }

    pub fn next_roll20(&mut self, value: u8) {
        self.next_rolls.push(value);
    }

    pub fn roll20(&mut self) -> u8 {
        let roll = self.next_rolls[self.index];
        self.index += 1;
        roll
    }
}


#[test]
fn roll_a_fixed_sequence_of_d20s() {
    let mut roller = FixedDiceRoller::new();
    roller.next_roll20(10);
    roller.next_roll20(15);
    roller.next_roll20(1);

    assert_eq!(roller.roll20(), 10);
    assert_eq!(roller.roll20(), 15);
    assert_eq!(roller.roll20(), 1);
}