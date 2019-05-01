use std::cell::RefCell;

use crate::move_predicate::{MovePredicate, WithPosition};

pub mod move_predicates;

pub struct Player<'a> {
    position: RefCell<(u32, u32)>,
    move_predicate: &'a MovePredicate,
}

impl<'a> WithPosition for Player<'a> {
    fn position(&self) -> (u32, u32) {
        self.position()
    }
}

impl<'a> Player<'a> {
    pub fn new(x: u32, y: u32, move_predicate: &'a MovePredicate) -> Player<'a> {
        Player {
            position: RefCell::new((x, y)),
            move_predicate,
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
