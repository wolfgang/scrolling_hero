use std::cell::RefCell;
use std::rc::Rc;

use crate::combat::Combatant;

pub type DungeonLayout = Vec<Vec<char>>;
pub type Position = (u32, u32);
pub type DungeonDefinition = (DungeonLayout, Position);
pub type CombatantRef = Rc<RefCell<Combatant>>;
