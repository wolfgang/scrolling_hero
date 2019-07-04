use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn stepping_on_health_potion_consumes_it() {
    let mut game = make_game_with_config(
        &game_with_player_hp(19),
        vec![
            "#@H#",
            "#..#",
        ]);

    game.on_key(Key::ArrowRight);
    game.on_key(Key::ArrowDown);

    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#.@#"
    ]);
}

fn game_with_player_hp(player_hp: u16) -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp,
        ..Default::default()
    }
}
