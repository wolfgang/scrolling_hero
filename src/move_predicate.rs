use crate::player::Player;

pub trait MovePredicate {
    fn can_move_left(&self, mover: &Player) -> bool;
    fn can_move_right(&self, mover: &Player) -> bool;
    fn can_move_down(&self, mover: &Player) -> bool;
}
