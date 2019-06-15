use console::Key;

use super::game_test_helpers::*;

#[ignore]
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

#[test]
fn render_player_hp() {
    let mut game = make_game(vec![
        "#.@#",
        "#..#"
    ]);

    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 100"]);
}


#[ignore]
#[test]
fn if_player_hits_guard_guard_will_hit_back_for_5_damage() {
    let mut game = make_game_with_camera_offset(100, vec![
        "#..#",
        "#G@#",
        "#G.#"
    ]);

    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 100"]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 95"]);

    game.on_key(Key::ArrowLeft);
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 95"]);

    game.on_key(Key::ArrowDown);
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 90"]);
}
