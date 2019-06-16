use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn player_onehits_guards_with_hp_1() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 1,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@G#",
        "#..G#",
    ]);

    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#G@G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowLeft);
    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#@.G#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowRight);
    game.on_key(Key::ArrowRight);
    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#..@#",
        "#..G#",
    ]);

    game.on_key(Key::ArrowDown);
    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#...#",
        "#..@#",
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
