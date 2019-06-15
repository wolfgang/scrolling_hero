use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn player_onehits_guards_with_hp_1() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 1,
    };

    let mut game = make_game_with_config(&config, vec![
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
        "#@.G#",
        "#..G#",
    ]);
}

#[test]
fn render_player_hp() {
    let mut game = make_game(vec![
        "#G@#",
        "#..#"
    ]);

    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 100"]);

    game.on_key(Key::ArrowLeft);

    // Guard hit us back, so HP are now two digits
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: \d{2}"]);

}
