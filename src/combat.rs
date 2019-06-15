use crate::game::state::GameState;
use crate::types::CombatantRef;

pub struct Combatant {
    pub hp: u16
}

impl Combatant {
    pub fn attack_simple(&self, target: &CombatantRef, damage: u16) {
        if self.hp > 0 {
            target.borrow_mut().hp -= damage;
        }
    }
}

pub fn resolve_simple(game_state: &mut GameState, pos: (usize, usize)) {
    let guard_ref = game_state.guard_at_ref(pos.0, pos.1);
    let player_ref = game_state.player_ref();

    player_ref.borrow().attack_simple(&guard_ref, 10);
    guard_ref.borrow().attack_simple(&player_ref, 5);
}
