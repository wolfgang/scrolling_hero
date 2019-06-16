use std::cell::RefCell;
use std::rc::Rc;

use crate::game::combatant::Combatant;
use crate::game::GameConfig;

use super::fixed_dice_roller::FixedDiceRoller;

#[test]
fn with_hp_constructs_guard_with_given_hp_and_zero_attack_and_defense() {
    let combatant = Combatant::with_hp(10);
    assert_eq!(combatant.hp, 10);
    assert_eq!(combatant.attack, 0);
    assert_eq!(combatant.defense, 0);
}

#[test]
fn from_game_config_takes_values_from_given_game_config() {
    let game_config = GameConfig { camera_offset: 1, guard_hp: 20 };

    let combatant = Combatant::from_game_config(&game_config);
    assert_eq!(combatant.hp, 20);
    assert_eq!(combatant.attack, 0);
    assert_eq!(combatant.defense, 0);
}


#[test]
fn attacker_misses_first_then_hits() {
    let mut dice_roller = FixedDiceRoller::new();

    let attacker = Combatant { hp: 100, attack: 5, defense: 0 };
    let target = Combatant { hp: 20, attack: 0, defense: 10 };
    let target_ref = Rc::new(RefCell::new(target));

    dice_roller.next_roll(20, 4); // 4 + attack (5) < target defense (10)
    dice_roller.next_roll(20, 6); // 4 + attack > target defense
    dice_roller.next_roll(10, 3); // Damage roll

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 20);

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 17);
}

#[test]
fn attacker_does_not_attack_if_they_are_dead() {
    let attacker = Combatant::with_hp(0);
    let target_ref = Combatant::ref_with_hp(10);

    let mut dice_roller = FixedDiceRoller::new();

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 10);
}