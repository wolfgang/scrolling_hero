use console::Key;

use crate::game::{Game, GameConfig};

use super::game_test_helpers::*;

#[test]
fn player_onehits_guards_with_hp_1() {
    let mut game = make_game_with_config(&game_with_weak_guards(), vec![
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
fn render_current_player_hp() {
    let config = game_with_strong_guards();
    let mut game = make_game_with_config(
        &config,
        vec![
            "#G@#",
            "#..#"
        ]);

    verify_player_hp_rendered(&mut game, config.player_hp as i16);

    game.on_key(Key::ArrowLeft);

    let new_player_hp = game.game_state.borrow_player().hp;
    assert!(new_player_hp < config.player_hp as i16);
    verify_player_hp_rendered(&mut game, new_player_hp);
}

#[test]
fn render_current_player_hp_after_non_combat_move() {
    let config = game_with_strong_guards();
    let mut game = make_game_with_config(
        &config,
        vec![
            "#.@#",
            "#..#"
        ]);

    verify_player_hp_rendered(&mut game, config.player_hp as i16);

    game.on_key(Key::ArrowLeft);

    verify_player_hp_rendered(&mut game, config.player_hp as i16);
}

fn verify_player_hp_rendered(game: &mut Game, player_hp: i16) {
    verify_lines_rendered_match(game, vec![&format!(r"\s+HP: {}$", player_hp)]);
}

#[test]
fn when_player_hits_guard_print_damage_dealt() {
    let config = game_with_strong_guards();

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@.#",
        "#...#"
    ]);

    game.on_key(Key::ArrowLeft);

    let damage_to_guard = config.guard_hp as i16 - game.game_state.borrow_guard_at((1, 1)).hp;
    let damage_to_player = config.player_hp as i16 - game.game_state.borrow_player().hp;

    verify_lines_rendered_match(&mut game, vec![
        r".*",
        &format!(r"\s+Player hits Guard for {}", damage_to_guard),
        &format!(r"\s+Guard hits Player for {}", damage_to_player)
    ]);
}


#[test]
fn when_attacks_miss_display_different_messages() {
    let mut game = make_game_with_config(
        &game_with_unhittable_combatants(),
        vec![
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
    let mut game = make_game_with_config(
        &game_with_weak_guards(),
        vec![
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
    let mut game = make_game_with_config(
        &game_with_strong_guards(),
        vec![
            "#...#",
            "#G@.#",
            "#...#"
        ]);

    game.on_key(Key::ArrowLeft);
    game.on_key(Key::ArrowRight);

    verify_lines_rendered_match(&mut game, vec![
        r"#...#.*",
        r"#G.@#$",
        r"#...#$"
    ]);
}

#[test]
fn game_is_over_if_player_dies() {
    let mut game = make_game_with_config(
        &game_with_weak_player(),
        vec![
            "#G@#",
            "#..#"
        ]);

    game.on_key(Key::ArrowLeft);
    assert!(game.game_state.borrow_player().hp <= 0);
    assert!(!game.is_running());
}


#[test]
fn guards_are_spawned_in_next_dungeon() {
    let mut game = make_game_with_two_dungeons(
        &game_with_strong_guards(),
        vec![
            "#.@#",
            "#.E#"
        ],
        vec![
            "#G@#",
            "#..#",
        ]);

    verify_dungeon_rendered(&mut game, vec![
        "#.@#",
        "#.E#"
    ]);


    game.on_key(Key::ArrowDown);

    verify_dungeon_rendered(&mut game, vec![
        "#G@#",
        "#..#"
    ]);

    game.on_key(Key::ArrowLeft);

    verify_dungeon_rendered(&mut game, vec![
        "#G@#",
        "#..#"
    ]);
}


fn game_with_strong_guards() -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 100,
        ..Default::default()
    }
}

fn game_with_weak_guards() -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 1,
        player_hp: 100,
        ..Default::default()
    }
}

fn game_with_unhittable_combatants() -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 100,
        // guard and player can never be hit
        guard_defense: 100,
        player_defense: 100,
        ..Default::default()
    }
}

fn game_with_weak_player() -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp: 1,
        ..Default::default()
    }
}
