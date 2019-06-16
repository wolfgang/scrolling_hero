use std::cell::RefCell;
use std::rc::Rc;

use crate::game::dice_roller::DiceRoller;
use crate::types::CombatantRef;

#[derive(Default)]
pub struct CombatantConfig {
    pub initial_hp: u16,
    pub attack: u8,
    pub defense: u8,
}

pub struct Combatant {
    pub hp: i16,
    pub attack: u8,
    pub defense: u8,
}

impl Combatant {
    pub fn with_config(config: &CombatantConfig) -> Combatant {
        Combatant {
            hp: config.initial_hp as i16,
            attack: config.attack,
            defense: config.defense,
        }
    }

    pub fn into_ref(self) -> CombatantRef {
        Rc::new(RefCell::new(self))
    }

    pub fn attack_simple(&self, target: &CombatantRef, damage: i16) {
        if self.hp > 0 {
            target.borrow_mut().hp -= damage;
        }
    }

    pub fn attack(&self, target: &CombatantRef, dice_roller: &mut DiceRoller) -> u8 {
        let mut damage = 0;
        if self.hp > 0 {
            let attack_roll = dice_roller.roll(20);
            if attack_roll + self.attack > target.borrow().defense {
                damage = dice_roller.roll(10);
                target.borrow_mut().hp -= damage as i16;
            }
        }
        damage
    }
}