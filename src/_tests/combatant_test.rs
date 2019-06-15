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
fn player_hits_guard_twice() {
    let mut dice_roller = FixedDiceRoller::new();

    let player = Combatant::with_hp(100);
    let guard = Combatant::with_hp(20);
    let guard_ref = Rc::new(RefCell::new(guard));

    dice_roller.next_roll(20, 11);
    dice_roller.next_roll(10, 7);
    dice_roller.next_roll(10, 4);

    player.attack(&guard_ref, &mut dice_roller);

    assert_eq!(guard_ref.borrow().hp, 13);

    player.attack(&guard_ref, &mut dice_roller);
    assert_eq!(guard_ref.borrow().hp, 9);
}

//#[test]
//fn player_misses_because_roll_is_too_low() {
//    let mut dice_roller = FixedDiceRoller::new();
//
//    let player = Combatant {hp: 100, attack: 5, defense: 10};
//    let guard = Combatant::ref_with_hp(20);
//
//}