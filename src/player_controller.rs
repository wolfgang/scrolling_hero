use console::Key;

use crate::player::Player0;

pub struct PlayerController<'a> {
    player: &'a Player0
}

impl<'a> PlayerController<'a> {
    pub fn new(player: &'a Player0) -> PlayerController {
        PlayerController { player }
    }

    pub fn on_key(&self, key: Key) -> bool {
        match key {
            Key::Escape => { return false; }
            Key::ArrowLeft => { self.player.move_left() }
            Key::ArrowRight => { self.player.move_right() }
            Key::ArrowDown => { self.player.move_down() }
            _ => {}
        }

        true
    }
}
