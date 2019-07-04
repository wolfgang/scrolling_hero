use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn stepping_on_health_option_consumes_it_and_increases_player_hp() {
    let mut game = make_game_with_config(
        &game_with_player_hp(19),
        vec![
            "#..#",
            "#@H#",
            "#..#",
        ]);

    verify_dungeon_rendered(&mut game, vec![
        "#..#",
        "#@H#",
        "#..#",
    ]);

    game.on_key(Key::ArrowRight);

    assert!(game.game_state.borrow_player().hp > 19);

    // Verify new hp is rendered .. has to be at least 20
    verify_lines_rendered_match(&mut game, vec![r"\s+HP: 2\d$"]);

    game.on_key(Key::ArrowDown);

    verify_dungeon_rendered(&mut game, vec![
        "#..#",
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
