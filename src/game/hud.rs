use crate::game::combatant::CombatResult;

pub struct Hud {
    lines: Vec<String>
}

impl Hud {
    pub fn new() -> Hud {
        Hud {
            lines: Vec::with_capacity(10)
        }
    }

    pub fn player_combat_message(combat_result: CombatResult) -> String {
        Hud::attack_message("Player", "Guard", combat_result)
    }

    pub fn guard_combat_message(combat_result: CombatResult) -> String {
        Hud::attack_message("Guard", "Player", combat_result)
    }

    pub fn attack_message(attacker: &str, target: &str, combat_result: CombatResult) -> String {
        if combat_result.attacker_dead {
            return String::from(format!("{} dies!", attacker));
        }

        if combat_result.damage_done > 0 {
            let action = if combat_result.is_crit { "CRITS" } else { "hits" };
            return String::from(
                format!("{} {} {} for {}", attacker, action, target, combat_result.damage_done)
            );
        }
        String::from(format!("{} misses {}!", attacker, target))
    }

    pub fn player_health_message(player_hp: i16) -> String {
        format!("HP: {}", player_hp)
    }
}