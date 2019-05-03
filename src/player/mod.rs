use std::cell::RefCell;
use std::rc::Rc;

use crate::move_predicate::{MovePredicate, WithPosition};

pub mod move_predicates;

pub struct Player2 {
    position: RefCell<(u32, u32)>,
    move_predicate: Rc<MovePredicate>,
}

impl Player2 {
    pub fn new(x: u32, y: u32, move_predicate: &Rc<MovePredicate>) -> Player2 {
        Player2 {
            position: RefCell::new((x, y)),
            move_predicate: Rc::clone(move_predicate),
        }
    }

    pub fn position(&self) -> (u32, u32) {
        *self.position.borrow()
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

impl WithPosition for Player2 {
    fn position(&self) -> (u32, u32) {
        self.position()
    }
}
