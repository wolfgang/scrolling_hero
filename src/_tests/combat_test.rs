use crate::_tests::fixed_dice_roller::FixedDiceRoller;
use crate::game::combat::Combatant;

#[test]
fn player_hits_guard_twice() {
    let mut dice_roller = FixedDiceRoller::new();

    let player = Combatant::with_hp(100);
    let guard_ref = Combatant::ref_with_hp(20);

    dice_roller.next_roll(20, 11);
    dice_roller.next_roll(10, 7);
    dice_roller.next_roll(10, 4);

    player.attack(&guard_ref, &mut dice_roller);

    assert_eq!(guard_ref.borrow().hp, 13);

    player.attack(&guard_ref, &mut dice_roller);
    assert_eq!(guard_ref.borrow().hp, 9);
}