use std::cell::RefCell;

use move_predicates::NonCollidingPlayerMovePredicate;

use crate::move_predicate::{MovePredicate, WithPosition};

pub mod move_predicates;

pub struct Player0 {
    position: RefCell<(u32, u32)>,
    move_predicate: Box<dyn MovePredicate>,
}

impl WithPosition for Player0 {
    fn position(&self) -> (u32, u32) {
        self.position()
    }
}

impl Player0 {
    pub fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Player0 {
        Player0 {
            position: RefCell::new((x, y)),
            move_predicate: Box::from(NonCollidingPlayerMovePredicate::new(max_x, max_y)),
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


pub struct Player2<'a> {
    position: RefCell<(u32, u32)>,
    move_predicate: &'a MovePredicate,
}

impl<'a> WithPosition for Player2<'a> {
    fn position(&self) -> (u32, u32) {
        self.position()
    }
}

impl<'a> Player2<'a> {
    pub fn new(x: u32, y: u32, move_predicate: &'a MovePredicate) -> Player2<'a> {
        Player2 {
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
