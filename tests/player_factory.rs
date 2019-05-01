use std::rc::Rc;

use sch::move_predicate::MovePredicate;
use sch::player::move_predicates::NonCollidingPlayerMovePredicate;
use sch::player::Player2;

pub fn without_bounds(x: u32, y: u32) -> Player2 {
    Player2::new(x, y, &make_predicate(1000000, 1000000))
}

pub fn with_bounds(x: u32, y: u32, max_x: u32, max_y: u32) -> Player2 {
    Player2::new(x, y, &make_predicate(max_x, max_y))
}

fn make_predicate(max_x: u32, max_y: u32) -> Rc<MovePredicate> {
    Rc::new(NonCollidingPlayerMovePredicate::new(max_x, max_y))
}