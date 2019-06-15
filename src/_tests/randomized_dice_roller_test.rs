use crate::game::dice_roller::DiceRoller;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;

#[test]
fn rolls_random_values_up_to_given_max() {
    let mut roller = RandomizedDiceRoller::new();

    let mut values = Vec::with_capacity(100);

    for _ in 0..100 {
        let roll = roller.roll(20);
        values.push(roll);
        // All rolls are between 1 and 20
        assert!(roll >= 1);
        assert!(roll <= 20);
    }

    // Contains low and high roll
    assert!(values.contains(&1));
    assert!(values.contains(&20));

    // Has at least 10 different values
    values.sort();
    values.dedup();
    assert!(values.len() > 10);
}