use std::cell::RefCell;
use std::rc::Rc;

use crate::game::dice_roller::DiceRoller;
use crate::types::{CombatantRef, Position};

use super::state::GameState;

pub struct Combatant {
    pub hp: i16
}

impl Combatant {
    pub fn ref_with_hp(hp: i16) -> CombatantRef {
        Rc::new(RefCell::new(Combatant::with_hp(hp)))
    }
    pub fn with_hp(hp: i16) -> Combatant {
        Combatant { hp }
    }

    pub fn attack_simple(&self, target: &CombatantRef, damage: i16) {
        if self.hp > 0 {
            target.borrow_mut().hp -= damage;
        }
    }

    pub fn attack(&self, target: &CombatantRef, dice_roller: &mut DiceRoller) {
        target.borrow_mut().hp -= dice_roller.roll(10) as i16;
    }
}

pub fn resolve_simple(game_state: &mut GameState, pos: Position) {
    let guard_ref = game_state.guard_ref_at(pos);
    let player_ref = game_state.player_ref();

    player_ref.borrow().attack_simple(&guard_ref, 10);
    guard_ref.borrow().attack_simple(&player_ref, 5);
}
