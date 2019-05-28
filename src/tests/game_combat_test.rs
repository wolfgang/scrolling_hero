use console::Key;

use crate::tests::game_test_helpers::*;

#[test]
fn player_collides_with_guard() {
    let mut game = make_game(vec![
        "#...#",
        "#G@G#",
        "#.G.#"
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#G@G#",
        "#.G.#"
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#G@G#",
        "#.G.#"
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#G@G#",
        "#.G.#"
    ]);

}
