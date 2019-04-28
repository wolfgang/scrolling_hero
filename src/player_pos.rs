use std::rc::Rc;
use std::cell::RefCell;

pub type PlayerPos = Rc<RefCell<(u32, u32)>>;

pub fn new(x: u32, y: u32) -> PlayerPos {
    Rc::new(RefCell::new((x, y)))
}