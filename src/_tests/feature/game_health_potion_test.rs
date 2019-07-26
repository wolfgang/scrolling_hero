use console::Key;

use crate::_tests::feature::game_test_helpers::*;
use crate::game::{Game, GameConfig};

#[test]
fn stepping_on_health_potion_displays_new_hp_and_how_much_was_gained() {
    let config = GameConfig {
        camera_offset: 100,
        player_hp: 100,
        player_defense: 1,
        guard_hp: 100,
        // Ensure that the guard always hits
        guard_attack: 20,
        ..Default::default()
    };

    let mut game = make_game_with_config(&config, vec![
        "#...#",
        "#G@.#",
        "#..H#",
    ]);


    // Guard attacks a couple of times
    // This is to make sure that we don't randomly heal more than max hp

    for _ in 0..10 {
        game.on_key(Key::ArrowLeft);
    }
    // Go down and suffer opportunity attack from guard before healing
    game.on_key(Key::ArrowDown);

    let player_hp_before_heal = game.get_player_hp();
    // We consume the health potion
    game.on_key(Key::ArrowRight);
    // Verify we are healed and the amount is displayed correctly
    let player_hp_after_heal = game.get_player_hp();
    assert!(player_hp_after_heal > player_hp_before_heal);

    verify_lines_rendered_match(&mut game, vec![
        &format!(r"\s+HP: {}$", player_hp_after_heal),
        &format!(r"\s+Player regains {} HP", player_hp_after_heal - player_hp_before_heal),
    ]);
}

#[test]
fn stepping_on_health_potion_removes_it() {
    let mut game = make_game_with_healthy_player(vec![
        "#...#",
        "#.@H#",
        "#...#",
    ]);

    game.on_key(Key::ArrowRight);
    game.on_key(Key::ArrowDown);

    verify_dungeon_rendered(&mut game, vec![
        "#...#",
        "#...#",
        "#..@#"
    ]);
}

fn make_game_with_healthy_player(dungeon: Vec<&str>) -> Game {
    make_game_with_config(
        &GameConfig {
            camera_offset: 100,
            player_hp: 100,
            ..Default::default()
        },
        dungeon)
}
