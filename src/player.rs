use std::cell::{Ref, RefCell};

pub struct Player {
    position: RefCell<(u32, u32)>,
    max_x: u32,
    max_y: u32,
}

impl Player {
    pub fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Player {
        Player {
            position: RefCell::new((x, y)),
            max_x,
            max_y,
        }
    }

    pub fn position(&self) -> Ref<(u32, u32)> {
        self.position.borrow()
    }

    pub fn move_left(&self) {
        if self.position.borrow().0 > 0 {
            self.position.borrow_mut().0 -= 1;
        }
    }

    pub fn move_right(&self) {
        if self.position.borrow().0 < self.max_x {
            self.position.borrow_mut().0 += 1;
        }
    }

    pub fn move_up(&self) {
        if self.position.borrow().1 > 0 {
            self.position.borrow_mut().1 -= 1;
        }
    }

    pub fn move_down(&self) {
        if self.position.borrow().1 < self.max_y {
            self.position.borrow_mut().1 += 1;
        }
    }
}
