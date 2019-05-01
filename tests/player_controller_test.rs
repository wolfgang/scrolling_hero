use console::Key;

use sch::mutable_position;
use sch::mutable_position::MutablePosition;
use sch::player_controller::PlayerController;

const MAX_X: u32 = 20;
const MAX_Y: u32 = 10;

#[test]
fn on_key_returns_false_for_escape_and_true_for_any_other_key() {
    let (_, player_controller) = setup_with_player_pos(1, 0);
    assert_eq!(false, player_controller.on_key(Key::Escape));
    assert_eq!(true, player_controller.on_key(Key::ArrowLeft));
    assert_eq!(true, player_controller.on_key(Key::ArrowRight));
    assert_eq!(true, player_controller.on_key(Key::ArrowDown));
}

#[test]
fn arrow_left_moves_player_left() {
    let (player_pos, player_controller) = setup_with_player_pos(8, 0);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 7, 0);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 6, 0);
}

#[test]
fn arrow_left_does_not_move_beyond_zero() {
    let (player_pos, player_controller) = setup_with_player_pos(1, 0);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 0, 0);
    player_controller.on_key(Key::ArrowLeft);
    assert_player_pos(&player_pos, 0, 0);

}

#[test]
fn arrow_right_moves_player_right() {
    let (player_pos, player_controller) = setup_with_player_pos(8, 0);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, 9, 0);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, 10, 0);
}

#[test]
fn arrow_right_does_not_move_beyond_given_max_x() {
    let (player_pos, player_controller) = setup_with_player_pos(MAX_X - 1, 0);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, MAX_X, 0);
    player_controller.on_key(Key::ArrowRight);
    assert_player_pos(&player_pos, MAX_X, 0);
}

#[test]
fn arrow_down_moves_player_down() {
    let (player_pos, player_controller) = setup_with_player_pos(8, 0);
    player_controller.on_key(Key::ArrowDown);
    assert_player_pos(&player_pos, 8, 1);
    player_controller.on_key(Key::ArrowDown);
    assert_player_pos(&player_pos, 8, 2);    
}

#[test]
fn arrow_down_does_not_move_beyond_given_max_y() {
    let (player_pos, player_controller) = setup_with_player_pos(8, MAX_Y - 1);
    player_controller.on_key(Key::ArrowDown);
    assert_player_pos(&player_pos, 8, MAX_Y);
    player_controller.on_key(Key::ArrowDown);
    assert_player_pos(&player_pos, 8, MAX_Y);    
}

fn setup_with_player_pos(player_x: u32, player_y: u32) -> (MutablePosition, PlayerController) {
    let player_pos = mutable_position::new(player_x, player_y);
    let player_controller = PlayerController::new(&player_pos, MAX_X, MAX_Y);
    (player_pos, player_controller)
}

fn assert_player_pos(player_pos: &MutablePosition, x: u32, y: u32) {
    assert_eq!(x, player_pos.borrow().0);
    assert_eq!(y, player_pos.borrow().1);
}