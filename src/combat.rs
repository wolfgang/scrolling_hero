pub struct Combatant {
    pub hp: u16
}

pub fn resolve_simple(player: &mut Combatant, guard: &mut Combatant) {
    guard.hp -= 10;
    if guard.hp > 0 { player.hp -= 5; }
}
