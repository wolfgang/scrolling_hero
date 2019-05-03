use std::rc::Rc;

use crate::move_predicate::{MovePredicate, WithPosition};
use crate::player::move_predicates::CompositePlayerMovePredicate;
use crate::player::Player;

struct MockPredicate {
    result: bool
}

impl MockPredicate {
    pub fn new(result: bool) -> MockPredicate {
        MockPredicate { result }
    }
}

impl MovePredicate for MockPredicate {
    fn can_move_left(&self, _mover: &WithPosition) -> bool {
        self.result
    }

    fn can_move_right(&self, _mover: &WithPosition) -> bool {
        self.result
    }

    fn can_move_down(&self, _mover: &WithPosition) -> bool {
        self.result
    }
}

#[test]
fn always_true_for_empty_composite() {
    let predicate: Rc<MovePredicate> = Rc::new(CompositePlayerMovePredicate::new());
    let player = Player::new(0, 0, &predicate);
    assert!(predicate.can_move_right(&player));
    assert!(predicate.can_move_left(&player));
    assert!(predicate.can_move_right(&player));
}

#[test]
fn returns_false_if_some_are_false() {
    let mut composite = CompositePlayerMovePredicate::new();
    let mock1: Rc<MovePredicate> = Rc::new(MockPredicate::new(true));
    let mock2: Rc<MovePredicate> = Rc::new(MockPredicate::new(false));
    composite.add(&mock1);
    composite.add(&mock2);
    let predicate: Rc<MovePredicate> = Rc::new(composite);
    let player = Player::new(0, 0, &predicate);
    assert_eq!(predicate.can_move_right(&player), false);
}

#[test]
fn returns_true_if_all_are_true() {
    let mut composite = CompositePlayerMovePredicate::new();
    let mock1: Rc<MovePredicate> = Rc::new(MockPredicate::new(true));
    let mock2: Rc<MovePredicate> = Rc::new(MockPredicate::new(true));
    composite.add(&mock1);
    composite.add(&mock2);
    let predicate: Rc<MovePredicate> = Rc::new(composite);
    let player = Player::new(0, 0, &predicate);
    assert_eq!(predicate.can_move_right(&player), true);
}