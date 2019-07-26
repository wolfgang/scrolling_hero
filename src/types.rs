use std::cell::RefCell;
use std::rc::Rc;

use crate::dungeon::provider::DungeonProvider;
use crate::game::combatant::Combatant;
use crate::game::dice_roller::DiceRoller;

pub type DungeonLayout = Vec<Vec<char>>;
pub type Position = (u32, u32);
pub type DungeonDefinition = (DungeonLayout, Position);
pub type CombatantRef = Rc<RefCell<Combatant>>;
pub type DungeonProviderRef = Rc<RefCell<DungeonProvider>>;
pub type DiceRollerRef = Rc<RefCell<dyn DiceRoller>>;
