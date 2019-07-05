use console::Key;

use crate::_tests::feature::game_test_helpers::*;
use crate::game::GameConfig;

#[test]
fn stepping_on_health_potion_increases_player_health() {
    let mut game = make_game_with_config(&game_with_initial_player_hp(100), vec![
        "#...#",
        "#.@H#",
        "#...#",
    ]);

    let player_ref = game.game_state.player_ref();
    player_ref.borrow_mut().apply_damage(20);
    assert_eq!(player_ref.borrow().hp, 80);

    game.on_key(Key::ArrowRight);

    assert!(player_ref.borrow().hp > 80);
}

#[test]
fn stepping_on_health_potion_removes_it() {
    let mut game = make_game_with_config(&game_with_initial_player_hp(100), vec![
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


fn game_with_initial_player_hp(player_hp: u16) -> GameConfig {
    GameConfig {
        camera_offset: 100,
        player_hp,
        ..Default::default()
    }
}
