use crate::game_state::GameState;

pub struct Combatant {
    pub hp: u16
}

impl Combatant {
    pub fn attack_simple() {}
}

pub fn resolve_simple(game_state: &mut GameState, pos: (usize, usize)) {
    let guard = game_state.guard_at_ref(pos.0, pos.1);
    let mut guard_ref = guard.borrow_mut();
    guard_ref.hp -= 10;
    if guard_ref.hp > 0 { game_state.player.hp -= 5; }

    if guard_ref.hp <= 0 {
        game_state.dungeon[pos.1][pos.0] = '.';
    }
}
