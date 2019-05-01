use std::cell::{Ref, RefCell};

trait MovePredicate {
    fn can_move_left(&self, mover: &Player) -> bool;
    fn can_move_right(&self, mover: &Player) -> bool;
    fn can_move_down(&self, mover: &Player) -> bool;
}

struct PlayerMovePredicate {
    max_x: u32,
    max_y: u32,
}

impl MovePredicate for PlayerMovePredicate {
    fn can_move_left(&self, mover: &Player) -> bool {
        mover.position().0 > 0
    }

    fn can_move_right(&self, mover: &Player) -> bool {
        mover.position().0 < self.max_x
    }

    fn can_move_down(&self, mover: &Player) -> bool {
        mover.position().1 < self.max_y
    }
}

impl PlayerMovePredicate {
    pub fn new(max_x: u32, max_y: u32) -> PlayerMovePredicate {
        PlayerMovePredicate {
            max_x,
            max_y,
        }
    }
}

pub struct Player {
    position: RefCell<(u32, u32)>,
    move_predicate: Box<dyn MovePredicate>,
}

impl Player {
    pub fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Player {
        Player {
            position: RefCell::new((x, y)),
            move_predicate: Box::from(PlayerMovePredicate::new(max_x, max_y)),
        }
    }

    pub fn position(&self) -> Ref<(u32, u32)> {
        self.position.borrow()
    }

    pub fn move_left(&self) {
        if self.move_predicate.can_move_left(self) {
            self.position.borrow_mut().0 -= 1;
        }
    }

    pub fn move_right(&self) {
        if self.move_predicate.can_move_right(self) {
            self.position.borrow_mut().0 += 1;
        }
    }

    pub fn move_down(&self) {
        if self.move_predicate.can_move_down(self) {
            self.position.borrow_mut().1 += 1;
        }
    }
}
