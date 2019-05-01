use std::cell::{Ref, RefCell};

struct Player {
    position: RefCell<(u32, u32)>,
    max_x: u32,
    max_y: u32,
}

impl Player {
    pub fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Player {
        Player {
            position: RefCell::new((x, y)),
            max_x,
            max_y,
        }
    }

    pub fn position(&self) -> Ref<(u32, u32)> {
        self.position.borrow()
    }

    pub fn move_left(&self) {
        if self.position.borrow().0 > 0 {
            self.position.borrow_mut().0 -= 1;
        }
    }

    pub fn move_right(&self) {
        if self.position.borrow().0 < self.max_x {
            self.position.borrow_mut().0 += 1;
        }
    }

    pub fn move_up(&self) {
        if self.position.borrow().1 > 0 {
            self.position.borrow_mut().1 -= 1;
        }
    }

    pub fn move_down(&self) {
        if self.position.borrow().1 < self.max_y {
            self.position.borrow_mut().1 += 1;
        }
    }
}


#[test]
fn has_initial_position() {
    let player = Player::new(10, 20, 100, 100);
    assert_eq!((10, 20), *player.position());
}

#[test]
fn move_left_until_zero_x() {
    let player = Player::new(2, 0, 100, 100);
    player.move_left();
    assert_eq!((1, 0), *player.position());
    player.move_left();
    assert_eq!((0, 0), *player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), *player.position());
}

#[test]
fn move_right_until_max_x() {
    let player = Player::new(8, 0, 10, 10);
    player.move_right();
    assert_eq!((9, 0), *player.position());
    player.move_right();
    assert_eq!((10, 0), *player.position());
    player.move_right();
    player.move_right();
    assert_eq!((10, 0), *player.position());
}

#[test]
fn move_up_until_zero_y() {
    let player = Player::new(0, 2, 100, 100);
    player.move_up();
    assert_eq!((0, 1), *player.position());
    player.move_up();
    assert_eq!((0, 0), *player.position());
    player.move_up();
    player.move_up();
    assert_eq!((0, 0), *player.position());
}

#[test]
fn move_down_until_max_y() {
    let player = Player::new(0, 8, 10, 10);
    player.move_down();
    assert_eq!((0, 9), *player.position());
    player.move_down();
    assert_eq!((0, 10), *player.position());
    player.move_down();
    player.move_down();
    assert_eq!((0, 10), *player.position());
}

#[test]
fn can_move_while_borrowed_immutably() {
    let player = Player::new(10, 20, 100, 100);
    move_player_right(&player);
    assert_eq!((11, 20), *player.position());
}

fn move_player_right(player: &Player) {
    player.move_right();
}
