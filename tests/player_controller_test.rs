use console::Key;
use speculate::speculate;

use sch::player::move_predicates::NonCollidingPlayerMovePredicate;
use sch::player::Player;
use sch::player_controller::PlayerController;

speculate! {

before {
    let predicate = NonCollidingPlayerMovePredicate::new(100, 200);
}

it "move_left" {
    let player = Player::new(10, 0, &predicate);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowLeft);
    assert_eq!((9, 0), player.position());
}

it "move_right" {
    let player = Player::new(10, 0, &predicate);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowRight);
    assert_eq!((11, 0), player.position());
}

it "move_down" {
    let player = Player::new(10, 0, &predicate);
    let player_controller = PlayerController::new(&player);
    player_controller.on_key(Key::ArrowDown);
    assert_eq!((10, 1), player.position());
}

it "on_key_returns_false_for_escape_and_true_for_any_other_key" {
    let player = Player::new(0, 0, &predicate);
    let player_controller = PlayerController::new(&player);
    assert_eq!(false, player_controller.on_key(Key::Escape));
    assert_eq!(true, player_controller.on_key(Key::ArrowLeft));
    assert_eq!(true, player_controller.on_key(Key::ArrowRight));
    assert_eq!(true, player_controller.on_key(Key::ArrowDown));
}

}