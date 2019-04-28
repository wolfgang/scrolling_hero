use console::{Key};

struct PlayerController {

}

impl PlayerController {
    pub fn new() -> PlayerController {
        PlayerController {}
    }

    pub fn on_key(&self, key: Key) -> bool {
        false
    }
}

#[test]
fn on_key_returns_false_when_escape_is_passed() {
    let player_controller = PlayerController::new();
    assert_eq!(false, player_controller.on_key(Key::Escape));
}