use std::rc::Rc;

use crate::move_predicate::{MovePredicate, WithPosition};
use crate::player::move_predicates::CompositePlayerMovePredicate;
use crate::player::Player;

struct MockPredicate {
    result: bool
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

//#[test]
//fn return_value_of_first_failing() {
//    let predicate = CompositePlayerMovePredicate::new();
//    let player = player_factory::without_bounds(0, 0);
//
//
//}