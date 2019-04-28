use console::{Key};
use std::rc::Rc;
use std::cell::{RefCell};

pub struct PlayerController {
    player_pos: Rc<RefCell<(u32, u32)>>,
    max_y: u32
}

impl PlayerController {
    pub fn new(player_pos: &Rc<RefCell<(u32, u32)>>, max_y: u32) -> PlayerController {
        PlayerController {
            player_pos: Rc::clone(player_pos),
            max_y: max_y

        }
    }

    pub fn on_key(&self, key: Key) -> bool {
        match key {
            Key::Escape => { return false }
            Key::ArrowLeft => {
                if self.player_pos.borrow().0 > 0 {
                    self.player_pos.borrow_mut().0 -= 1;
                }
            }
            Key::ArrowRight => {
                self.player_pos.borrow_mut().0 += 1;
            }
            Key::ArrowDown => {
                if self.player_pos.borrow().1 < self.max_y {
                    self.player_pos.borrow_mut().1 += 1;
                }
            }

            _ => { }
        }

        return true;
    }
}
