
mod player_factory;

const MAX_X: u32 = 10;
const MAX_Y: u32 = 20;


#[test]
fn player2_has_initial_position() {
    let player = player_factory::without_bounds(10, 20);
    assert_eq!((10, 20), player.position());
}


#[test]
fn move_left_until_zero_x() {
    let player = player_factory::without_bounds(2, 0);
    player.move_left();
    assert_eq!((1, 0), player.position());
    player.move_left();
    assert_eq!((0, 0), player.position());
    player.move_left();
    player.move_left();
    assert_eq!((0, 0), player.position());
}

#[test]
fn move_right_until_max_x() {
    let player = player_factory::with_bounds(MAX_X - 2, 0, MAX_X, MAX_Y);
    player.move_right();
    assert_eq!((MAX_X - 1, 0), player.position());
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
    player.move_right();
    player.move_right();
    assert_eq!((MAX_X, 0), player.position());
}
