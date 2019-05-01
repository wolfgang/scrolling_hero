use std::rc::Rc;
use std::cell::RefCell;

pub type MutablePosition = Rc<RefCell<(u32, u32)>>;

pub fn new(x: u32, y: u32) -> MutablePosition {
    Rc::new(RefCell::new((x, y)))
}