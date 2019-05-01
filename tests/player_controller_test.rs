use console::Key;

use sch::player::Player;

struct PlayerController<'a> {
    player: &'a Player
}

impl<'a> PlayerController<'a> {
    pub fn new(player: &'a Player) -> PlayerController {
        PlayerController { player }
    }

    pub fn on_key(&self, key: Key) {
        match key {
            Key::ArrowLeft => {
                self.player.move_left();
            }
            Key::ArrowRight => {
                self.player.move_right();
            }

            Key::ArrowDown => {
                self.player.move_down();
            }

            _ => {}
        }
    }
}

#[test]
fn move_left() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowLeft);
    assert_eq!((9, 0), *player.position());
}

#[test]
fn move_right() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowRight);
    assert_eq!((11, 0), *player.position());
}

#[test]
fn move_down() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowDown);
    assert_eq!((10, 1), *player.position());
}

fn player_at(x: u32, y: u32) -> Player {
    Player::new(x, y, 100, 200)
}