use std::cell::RefCell;
use std::rc::Rc;

use crate::game::combatant::{Combatant, CombatantConfig};

use super::fixed_dice_roller::FixedDiceRoller;

#[test]
fn with_config_takes_values_from_given_config() {
    let config = CombatantConfig { initial_hp: 20, attack: 5, defense: 10 };

    let combatant = Combatant::with_config(&config);
    assert_eq!(combatant.hp, 20);
    assert_eq!(combatant.attack, 5);
    assert_eq!(combatant.defense, 10);
}


#[test]
fn attacker_misses_first_then_hits() {
    let mut dice_roller = FixedDiceRoller::new();

    let attacker = Combatant::with_config(&CombatantConfig { initial_hp: 100, attack: 5, defense: 0 });
    let target = Combatant::with_config(&CombatantConfig { initial_hp: 20, attack: 0, defense: 10 });
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
fn if_attacker_rolls_20_damage_is_rolled_twice() {
    let mut dice_roller = FixedDiceRoller::new();

    let attacker = Combatant::with_config(&CombatantConfig { initial_hp: 100, attack: 5, defense: 0 });
    let target = Combatant::with_config(&CombatantConfig { initial_hp: 20, attack: 0, defense: 10 });
    let target_ref = Rc::new(RefCell::new(target));

    dice_roller.next_roll(20, 20);
    dice_roller.next_roll(10, 3); // Damage roll 1
    dice_roller.next_roll(10, 2); // Damage roll 2

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 15);
}

#[test]
fn attacker_does_not_attack_if_they_are_dead() {
    let attacker = combatant_with_initial_hp(0);
    let target_ref = combatant_with_initial_hp(10).into_ref();

    let mut dice_roller = FixedDiceRoller::new();

    attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(target_ref.borrow().hp, 10);
}

#[test]
fn attack_returns_damage_dealt_and_if_it_was_a_crit() {
    let attacker = combatant_with_initial_hp(10);
    let target_ref = combatant_with_initial_hp(20).into_ref();

    let mut dice_roller = FixedDiceRoller::new();
    dice_roller.next_roll(20, 15);
    dice_roller.next_roll(20, 20); // crit roll
    dice_roller.next_roll(10, 7);
    dice_roller.next_roll(10, 8); // double damage from crit roll
    dice_roller.next_roll(10, 9);

    let normal_result = attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(normal_result, (7, false));

    let crit_result = attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(crit_result, (17, true));
}

#[test]
fn attack_returns_zero_if_it_does_not_hit() {
    let attacker = combatant_with_initial_hp(10);
    let target_ref = Combatant::with_config(&CombatantConfig { initial_hp: 20, defense: 10, attack: 0 }).into_ref();

    let mut dice_roller = FixedDiceRoller::new();
    dice_roller.next_roll(20, 1);

    let (damage, _) = attacker.attack(&target_ref, &mut dice_roller);
    assert_eq!(damage, 0);
}

#[test]
fn heal_heals_with_d10_roll() {
    let mut combatant = combatant_with_initial_hp(20);
    combatant.apply_damage(10);
    let mut dice_roller = FixedDiceRoller::new();
    dice_roller.next_roll(10, 2);
    dice_roller.next_roll(10, 7);

    let result1 = combatant.heal(&mut dice_roller);
    assert_eq!(combatant.hp, 12);
    let result2 = combatant.heal(&mut dice_roller);
    assert_eq!(combatant.hp, 19);

    assert_eq!(result1, 2);
    assert_eq!(result2, 7);
}

#[test]
fn heal_caps_at_max_hp() {
    let mut combatant = combatant_with_initial_hp(50);
    combatant.apply_damage(3);
    let mut dice_roller = FixedDiceRoller::new();
    dice_roller.next_roll(10, 5);
    dice_roller.next_roll(10, 7);

    let result1 = combatant.heal(&mut dice_roller);
    let result2 = combatant.heal(&mut dice_roller);
    assert_eq!(combatant.hp, 50);
    assert_eq!(result1, 3);
    assert_eq!(result2, 0);
}


fn combatant_with_initial_hp(hp: u16) -> Combatant {
    Combatant::with_config(&CombatantConfig { initial_hp: hp, ..Default::default() })
}