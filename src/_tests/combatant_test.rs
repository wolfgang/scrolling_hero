use std::cell::RefCell;
use std::rc::Rc;

use crate::game::combat::Combatant;

use super::fixed_dice_roller::FixedDiceRoller;

#[test]
fn combatant_has_default_attack_and_defense_of_zero() {
    let combatant = Combatant::with_hp(10);
    assert_eq!(combatant.hp, 10);
    assert_eq!(combatant.attack, 0);
    assert_eq!(combatant.defense, 0);
}

#[test]
fn player_misses_first_then_hits() {
    let mut dice_roller = FixedDiceRoller::new();

    let attacker = Combatant { hp: 100, attack: 5, defense: 0 };
    let target = Combatant { hp: 20, attack: 0, defense: 10 };
    let target_ref = Rc::new(RefCell::new(target));

    dice_roller.next_roll(20, 4);
    dice_roller.next_roll(20, 6);
    dice_roller.next_roll(10, 3);


    attacker.attack(&target_ref, &mut dice_roller);

    assert_eq!(target_ref.borrow().hp, 20);

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 17);



}