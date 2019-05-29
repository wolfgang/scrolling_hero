use console::Key;

use crate::tests::game_test_helpers::*;

#[test]
fn player_kills_guard_after_colliding_twice() {
    let mut game = make_game_with_camera_offset(100, vec![
        "#...#",
        "#G@G#",
        "#..G#",
    ]);

    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#G@G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#G@G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#@.G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#.@G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#.@G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowRight);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#..@#",
        "#..G#",
    ]);


    game.on_key(Key::ArrowDown);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#..@#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered_start_with(&mut game, vec![
        "#...#",
        "#...#",
        "#..@#",
    ]);

}
