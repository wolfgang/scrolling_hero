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
fn returns_false_if_some_or_all_are_false() {
    let (player1, predicate1) = with_mock_results(true, false);
    assert_eq!(predicate1.can_move_right(&player1), false);
    let (player2, predicate2) = with_mock_results(false, true);
    assert_eq!(predicate2.can_move_right(&player2), false);
    let (player3, predicate3) = with_mock_results(false, false);
    assert_eq!(predicate3.can_move_right(&player3), false);

}

#[test]
fn returns_true_if_all_are_true() {
    let (player, predicate) = with_mock_results(true, true);
    assert_eq!(predicate.can_move_right(&player), true);
}

fn with_mock_results(result1: bool, result2: bool) -> (Player, Rc<MovePredicate>) {
    let mut composite = CompositePlayerMovePredicate::new();
    composite.add(&mock_predicate(result1));
    composite.add(&mock_predicate(result2));
    let predicate: Rc<MovePredicate> = Rc::new(composite);
    let player = Player::new(0, 0, &predicate);
    (player, predicate)
}

fn mock_predicate(result: bool) -> Rc<MovePredicate> {
    Rc::new(MockPredicate::new(result))
}
