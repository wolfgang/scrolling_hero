use crate::types::{CombatantRef, Position};

use super::state::GameState;

pub struct Combatant {
    pub hp: i16
}

impl Combatant {
    pub fn attack_simple(&self, target: &CombatantRef, damage: i16) {
        if self.hp > 0 {
            target.borrow_mut().hp -= damage;
        }
    }
}

pub fn resolve_simple(game_state: &mut GameState, pos: Position) {
    let guard_ref = game_state.guard_ref_at(pos);
    let player_ref = game_state.player_ref();

    player_ref.borrow().attack_simple(&guard_ref, 10);
    guard_ref.borrow().attack_simple(&player_ref, 5);
}
