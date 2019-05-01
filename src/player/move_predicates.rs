use crate::move_predicate::{MovePredicate, WithPosition};

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
