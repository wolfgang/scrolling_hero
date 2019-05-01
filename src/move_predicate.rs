pub trait WithPosition {
    fn position(&self) -> (u32, u32);
}

pub trait MovePredicate {
    fn can_move_left(&self, mover: &WithPosition) -> bool;
    fn can_move_right(&self, mover: &WithPosition) -> bool;
    fn can_move_down(&self, mover: &WithPosition) -> bool;
}
