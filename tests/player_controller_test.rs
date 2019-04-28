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
            }
            Key::ArrowRight => {
                self.player_pos.borrow_mut().0 += 1;
            }

            _ => { }
        }

        return true;

    }
}

#[test]
fn on_key_returns_false_for_escape_and_true_for_any_other_key() {
    let player_pos = make_player_pos(1, 0);
    let player_controller = PlayerController::new(&player_pos);
    assert_eq!(false, player_controller.on_key(Key::Escape));
    assert_eq!(true, player_controller.on_key(Key::ArrowLeft));
    assert_eq!(true, player_controller.on_key(Key::ArrowRight));
    assert_eq!(true, player_controller.on_key(Key::ArrowDown));
}

#[test]
fn arrow_left_moves_player_left() {
    let player_pos = make_player_pos(8, 0);
    let player_controller = PlayerController::new(&player_pos);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 7, 0);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 6, 0);
}

#[test]
fn arrow_right_moves_player_right() {
    let player_pos = make_player_pos(8, 0);
    let player_controller = PlayerController::new(&player_pos);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, 9, 0);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, 10, 0);
}


fn make_player_pos(x: u32, y: u32) -> Rc<RefCell<(u32, u32)>> {
    Rc::new(RefCell::new((x, y)))
}

fn assert_player_pos(player_pos: &Rc<RefCell<(u32, u32)>>, x: u32, y: u32) {
    assert_eq!(x, player_pos.borrow().0);
    assert_eq!(y, player_pos.borrow().1);
}