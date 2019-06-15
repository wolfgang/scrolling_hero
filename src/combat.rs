use crate::game_state::GameState;
use crate::types::CombatantRef;

pub struct Combatant {
    pub hp: u16
}

impl Combatant {
    pub fn attack_simple(&self, target: &CombatantRef, damage: u16) {
        target.borrow_mut().hp -= damage;
    }
}

pub fn resolve_simple(game_state: &mut GameState, pos: (usize, usize)) {
    let guard = game_state.guard_at_ref(pos.0, pos.1);
    game_state.player.attack_simple(&guard, 10);

    let guard_ref = guard.borrow();
    if guard_ref.hp > 0 { game_state.player.hp -= 5; }

    if guard_ref.hp <= 0 {
        game_state.dungeon[pos.1][pos.0] = '.';
    }
}
