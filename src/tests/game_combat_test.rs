use console::Key;

use crate::tests::game_test_helpers::*;

#[test]
fn player_is_moved_left_right_by_cursor_keys() {
    let mut game = make_game(vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#@.#",
        "#..#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#..#",
        "#.@#",
        "#..#"
    ]);
}
