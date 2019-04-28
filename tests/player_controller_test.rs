use console::{Key};
use std::rc::Rc;
use std::cell::{RefCell};

struct PlayerController {
    player_pos: Rc<RefCell<(u32, u32)>>
}

impl PlayerController {
    pub fn new(player_pos: &Rc<RefCell<(u32, u32)>>) -> PlayerController {
        PlayerController {
            player_pos: Rc::clone(player_pos)
        }
    }

    pub fn on_key(&self, key: Key) -> bool {
        match key {
            Key::Escape => { return false }
            Key::ArrowLeft => {
                self.player_pos.borrow_mut().0 -= 1;
                return true;
            }
            _ => { return true }
        }
    }
}

#[test]
fn on_key_returns_false_when_escape_is_passed() {
    let player_pos = Rc::new(RefCell::new((8, 0)));
    let player_controller = PlayerController::new(&player_pos);
    assert_eq!(false, player_controller.on_key(Key::Escape));
}

#[test]
fn on_key_returns_true_for_any_other_key() {
    let player_pos = Rc::new(RefCell::new((8, 0)));
    let player_controller = PlayerController::new(&player_pos);
    assert_eq!(true, player_controller.on_key(Key::ArrowLeft));
    assert_eq!(true, player_controller.on_key(Key::ArrowRight));
    assert_eq!(true, player_controller.on_key(Key::ArrowDown));
}

#[test]
fn arrow_left_moves_player_left() {
    let player_pos = Rc::new(RefCell::new((8, 0)));
    let player_controller = PlayerController::new(&player_pos);
    player_controller.on_key(Key::ArrowLeft);
    assert_eq!(7, player_pos.borrow().0);
    assert_eq!(0, player_pos.borrow().1);
    player_controller.on_key(Key::ArrowLeft);
    assert_eq!(6, player_pos.borrow().0);
    assert_eq!(0, player_pos.borrow().1);

}