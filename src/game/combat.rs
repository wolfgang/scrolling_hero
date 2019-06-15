use std::cell::RefCell;
use std::rc::Rc;

use crate::game::dice_roller::DiceRoller;
use crate::types::{CombatantRef, Position};

use super::state::GameState;

pub struct Combatant {
    pub hp: i16,
    pub attack: u8,
    pub defense: u8,
}

impl Combatant {
    pub fn ref_with_hp(hp: i16) -> CombatantRef {
        Rc::new(RefCell::new(Combatant::with_hp(hp)))
    }
    pub fn with_hp(hp: i16) -> Combatant {
        Combatant { hp, attack: 0, defense: 0 }
    }

    pub fn attack_simple(&self, target: &CombatantRef, damage: i16) {
        if self.hp > 0 {
            target.borrow_mut().hp -= damage;
        }
    }

    pub fn attack(&self, target: &CombatantRef, dice_roller: &mut DiceRoller) {
        if self.hp > 0 {
            let attack_roll = dice_roller.roll(20);
            if attack_roll + self.attack > target.borrow().defense {
                let damage_roll = dice_roller.roll(10);
                target.borrow_mut().hp -= damage_roll as i16;
            }
        }
    }
}

pub fn resolve(game_state: &mut GameState, pos: Position, dice_roller: &mut DiceRoller) {
    game_state.resolve_combat(pos, dice_roller);
}
