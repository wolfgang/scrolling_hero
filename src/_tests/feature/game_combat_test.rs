use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn player_onehits_guards_with_hp_1() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 1,
        player_hp: 100,
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
    let mut game = make_game_with_config(
        &GameConfig { player_hp: 100, ..Default::default() },
        vec![
            "#G@#",
            "#..#"
        ]);

    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 100"]);

    game.on_key(Key::ArrowLeft);

    // Guard hit us back, so HP are now two digits
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: \d{2}"]);
}

#[test]
fn when_player_hits_guard_print_damage_dealt() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 100,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@.#",
        "#...#"
    ]);

    game.on_key(Key::ArrowLeft);

    let damage_to_guard = 50 - game.game_state.borrow_guard_at((1, 1)).hp;
    let damage_to_player = 100 - game.game_state.borrow_player().hp;

    verify_lines_rendered_match(&mut game, vec![
        r".*",
        &format!(r"\s+Player hits Guard for {}", damage_to_guard),
        &format!(r"\s+Guard hits Player for {}", damage_to_player)
    ]);
}


#[test]
fn when_attacks_miss_display_different_messages() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 100,
        // guard and player can never be hit
        guard_defense: 100,
        player_defense: 100,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@.#",
        "#...#"
    ]);

    game.on_key(Key::ArrowLeft);

    verify_lines_rendered_match(&mut game, vec![
        r".*",
        r"\s+Player misses Guard!",
        r"\s+Guard misses Player!"
    ]);
}

#[test]
fn display_guard_dies_if_guard_drops_below_zero() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 1,
        player_hp: 100,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#.@G#",
        "#...#",
        "#...#",
    ]);


    game.on_key(Key::ArrowRight);

    verify_lines_rendered_match(&mut game, vec![
        r".*",
        r".*",
        r"\s+Guard dies!"
    ]);
}


#[test]
fn when_player_moves_away_clear_combat_log() {
    let config = GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 100,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@.#",
        "#...#"
    ]);

    game.on_key(Key::ArrowLeft);
    game.on_key(Key::ArrowRight);

    verify_lines_rendered_match(&mut game, vec![
        r"#...#.*",
        r"#G.@#",
        r"#...#"
    ]);
}