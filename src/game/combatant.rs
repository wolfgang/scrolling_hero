use std::cell::RefCell;
use std::cmp::min;
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
    max_hp: i16,
    pub attack: u8,
    pub defense: u8,
}

impl Combatant {
    pub fn with_config(config: &CombatantConfig) -> Combatant {
        Combatant {
            hp: config.initial_hp as i16,
            max_hp: config.initial_hp as i16,
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

    pub fn attack(&self, target: &CombatantRef, dice_roller: &mut dyn DiceRoller) -> (u8, bool) {
        let mut damage = 0;
        let mut is_crit = false;
        if self.hp > 0 {
            let attack_base_roll = dice_roller.roll(20);
            if target.borrow().is_hit(attack_base_roll + self.attack) {
                damage = dice_roller.roll(10);
                if attack_base_roll == 20 {
                    is_crit = true;
                    damage += dice_roller.roll(10)
                };
                target.borrow_mut().apply_damage(damage)
            }
        }
        (damage, is_crit)
    }

    pub fn heal(&mut self, dice_roller: &mut dyn DiceRoller) -> u8 {
        let roll = dice_roller.roll(10);
        let hp_before_heal = self.hp;
        self.hp = min(self.max_hp, self.hp + roll as i16);
        (self.hp - hp_before_heal) as u8

    }

    pub fn is_hit(&self, attack_roll: u8) -> bool {
        return attack_roll > self.defense;
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.hp -= damage as i16;
    }
}