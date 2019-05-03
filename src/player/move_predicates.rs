use std::rc::Rc;

use crate::move_predicate::{MovePredicate, WithPosition};
use crate::types::Dungeon;

pub struct NonCollidingPlayerMovePredicate {
    max_x: u32,
    max_y: u32,
}

impl MovePredicate for NonCollidingPlayerMovePredicate {
    fn can_move_left(&self, mover: &WithPosition) -> bool {
        mover.position().0 > 0
    }

    fn can_move_right(&self, mover: &WithPosition) -> bool {
        mover.position().0 < self.max_x
    }

    fn can_move_down(&self, mover: &WithPosition) -> bool {
        mover.position().1 < self.max_y
    }
}

impl NonCollidingPlayerMovePredicate {
    pub fn new(max_x: u32, max_y: u32) -> NonCollidingPlayerMovePredicate {
        NonCollidingPlayerMovePredicate {
            max_x,
            max_y,
        }
    }
}

pub struct WallCollidingPlayerMovePredicate {
    dungeon: Rc<Dungeon>
}

impl<'a> WallCollidingPlayerMovePredicate {
    pub fn new(dungeon: &Rc<Dungeon>) -> WallCollidingPlayerMovePredicate {
        WallCollidingPlayerMovePredicate { dungeon: Rc::clone(dungeon) }
    }
}

impl MovePredicate for WallCollidingPlayerMovePredicate {
    fn can_move_left(&self, mover: &WithPosition) -> bool {
        let (x, y) = mover.position();
        self.dungeon[y as usize][x as usize - 1] != 1
    }

    fn can_move_right(&self, mover: &WithPosition) -> bool {
        let (x, y) = mover.position();
        self.dungeon[y as usize][x as usize + 1] != 1
    }


    fn can_move_down(&self, mover: &WithPosition) -> bool {
        let (x, y) = mover.position();
        self.dungeon[y as usize + 1][x as usize] != 1
    }
}

