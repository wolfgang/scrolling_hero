use crate::game_state::GameState;

pub struct Combatant {
    pub hp: u16
}

pub fn resolve_simple(game_state: &mut GameState, pos: (usize, usize)) {
    let current_hp;
    {
        let mut guard = game_state.guard_at_mut(pos.0, pos.1);
        guard.hp -= 10;
        current_hp = guard.hp;
    }
    if current_hp > 0 { game_state.player.hp -= 5; }

    if current_hp <= 0 {
        game_state.dungeon[pos.1][pos.0] = '.';
    }

}
