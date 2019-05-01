use sch::player::Player;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;

#[test]
fn has_initial_position() {
    let player = Player::new(10, 20, MAX_X, MAX_Y);
    assert_eq!((10, 20), *player.position());
}

#[test]
fn move_left_until_zero_x() {
    let player = Player::new(2, 0, MAX_Y, MAX_Y);
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
    let player = Player::new(MAX_X - 2, 0, MAX_X, MAX_Y);
    player.move_right();
    assert_eq!((MAX_X - 1, 0), *player.position());
    player.move_right();
    assert_eq!((MAX_X, 0), *player.position());
    player.move_right();
    player.move_right();
    assert_eq!((MAX_X, 0), *player.position());
}

#[test]
fn move_up_until_zero_y() {
    let player = Player::new(0, 2, MAX_X, MAX_Y);
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
    let player = Player::new(0, MAX_Y - 2, MAX_X, MAX_Y);
    player.move_down();
    assert_eq!((0, MAX_Y - 1), *player.position());
    player.move_down();
    assert_eq!((0, MAX_Y), *player.position());
    player.move_down();
    player.move_down();
    assert_eq!((0, MAX_Y), *player.position());
}

#[test]
fn can_move_while_borrowed_immutably() {
    let player = Player::new(0, 0, MAX_X, MAX_Y);
    move_player_right(&player);
    assert_eq!((1, 0), *player.position());
}

fn move_player_right(player: &Player) {
    player.move_right();
}
