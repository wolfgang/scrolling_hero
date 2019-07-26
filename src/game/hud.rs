use std::cell::RefCell;

use crate::game::combatant::CombatResult;

pub struct Hud {
    pub lines: Vec<String>,
    combat_log: RefCell<Vec<String>>

}

impl Hud {
    pub fn new() -> Hud {
        Hud {
            lines: Vec::with_capacity(10),
            combat_log: RefCell::new(Vec::with_capacity(2)),

        }
    }

    pub fn refresh(&mut self, player_hp: i16) {
        self.lines.clear();
        self.lines.push(Hud::player_health_message(player_hp));
        self.lines.append(self.combat_log.borrow_mut().as_mut());
    }

    pub(crate) fn add_combat_log(&self, message: String) {
        self.combat_log.borrow_mut().push(message);
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