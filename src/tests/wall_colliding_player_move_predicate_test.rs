use std::rc::Rc;

use crate::move_predicate::{MovePredicate, WithPosition};
use crate::player::Player;

type Dungeon = Vec<Vec<u16>>;


struct WallCollidingPlayerMovePredicate {
    dungeon: Rc<Dungeon>
}

impl<'a> WallCollidingPlayerMovePredicate {
    pub fn new(dungeon: Rc<Dungeon>) -> WallCollidingPlayerMovePredicate {
        WallCollidingPlayerMovePredicate { dungeon }
    }
}

impl MovePredicate for WallCollidingPlayerMovePredicate {
    fn can_move_left(&self, mover: &WithPosition) -> bool {
        true
    }

    fn can_move_right(&self, mover: &WithPosition) -> bool {
        let (x, y) = mover.position();
        self.dungeon[y as usize][x as usize + 1] != 1
    }


    fn can_move_down(&self, mover: &WithPosition) -> bool {
        true
    }
}

#[test]
fn can_not_move_right_if_wall_is_to_the_right() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0]
    ];

    let predicate: Rc<MovePredicate> = Rc::new(WallCollidingPlayerMovePredicate::new(Rc::new(dungeon)));
    let player = Player::new(1, 1, &predicate);

    player.move_right();
    assert_eq!((1, 1), player.position());
}

#[test]
fn can_move_right_if_no_wall_is_to_the_right() {
    let dungeon = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0]
    ];
    let predicate: Rc<MovePredicate> = Rc::new(WallCollidingPlayerMovePredicate::new(Rc::new(dungeon)));
    let player = Player::new(1, 1, &predicate);

    player.move_right();
    assert_eq!((2, 1), player.position());
}
