use console::Key;

use crate::_tests::feature::game_test_helpers::*;
use crate::game::{Game, GameConfig};

#[test]
fn stepping_on_health_potion_increases_player_health() {
    let mut game = make_game_with_healthy_player(vec![
        "#...#",
        "#.@H#",
        "#...#",
    ]);

    let player_ref = game.game_state.player_ref();
    player_ref.borrow_mut().apply_damage(20);
    assert_eq!(player_ref.borrow().hp, 80);

    game.on_key(Key::ArrowRight);

    let player_hp = player_ref.borrow().hp;

    assert!(player_hp > 80);

    verify_lines_rendered_match(&mut game, vec![
        &format!(r"\s+HP: {}$", player_hp),
        &format!(r"\s+Player regains {} HP", player_hp - 80),
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
