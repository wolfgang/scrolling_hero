use console::Key;

use sch::player::Player0;
use sch::player_controller::PlayerController;

#[test]
fn move_left() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowLeft);
    assert_eq!((9, 0), player.position());
}

#[test]
fn move_right() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowRight);
    assert_eq!((11, 0), player.position());
}

#[test]
fn move_down() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowDown);
    assert_eq!((10, 1), player.position());
}

#[test]
fn on_key_returns_false_for_escape_and_true_for_any_other_key() {
    let player = player_at(10, 0);
    let player_controller = PlayerController::new(&player);
    assert_eq!(false, player_controller.on_key(Key::Escape));
    assert_eq!(true, player_controller.on_key(Key::ArrowLeft));
    assert_eq!(true, player_controller.on_key(Key::ArrowRight));
    assert_eq!(true, player_controller.on_key(Key::ArrowDown));
}


fn player_at(x: u32, y: u32) -> Player0 {
    Player0::new(x, y, 100, 200)
}