use console::Key;

use crate::game::GameConfig;

use super::game_test_helpers::*;

#[test]
fn stepping_on_health_option_increases_player_hp() {
    let mut game = make_game_with_config(
        &game_with_player_hp(19),
        vec![
            "#@H#",
            "#..#",
        ]);

    verify_dungeon_rendered(&mut game, vec![
        "#@H#",
        "#..#",
    ]);

    game.on_key(Key::ArrowRight);

    // Verify new hp is rendered as at least 20
    verify_lines_rendered_match(
        &mut game,
        vec![
            r"#.@#\s+HP: 2\d$",
            r"#..#",
        ],
    );
}

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

#[ignore]
#[test]
fn health_potion_adds_random_hp_between_1_and_10() {
    let initial_player_hp: i16 = 10;
    let mut game = make_game_with_config(
        &game_with_player_hp(initial_player_hp as u16),
        vec![
            "#@H#",
            "#.H#",
            "#.H#",
            "#.H#",
        ]);


    game.on_key(Key::ArrowRight);
    let player_hp_after_first_heal = game.game_state.borrow_player().hp;
    game.on_key(Key::ArrowDown);
    game.on_key(Key::ArrowDown);
    game.on_key(Key::ArrowDown);
    let player_hp_after_last_heal = game.game_state.borrow_player().hp;
    let first_heal = player_hp_after_first_heal - initial_player_hp;
    let last_heal = player_hp_after_last_heal - player_hp_after_first_heal;

    assert!(first_heal >= 1 && first_heal <= 10);
    assert!(last_heal >= 1 && last_heal <= 10);

    assert_ne!(first_heal, last_heal);
}


fn game_with_player_hp(player_hp: u16) -> GameConfig {
    GameConfig {
        camera_offset: 100,
        guard_hp: 50,
        player_hp,
        ..Default::default()
    }
}
