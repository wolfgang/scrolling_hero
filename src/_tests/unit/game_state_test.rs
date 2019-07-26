use crate::dungeon::helpers::make_dungeon;
use crate::game::GameConfig;
use crate::game::state::GameState;

#[test]
fn from_game_config_inits_player_and_guards() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@.G#"]);
    let game_config = GameConfig {
        guard_hp: 30,
        guard_attack: 7,
        guard_defense: 12,
        player_hp: 120,
        player_attack: 9,
        player_defense: 15,
        ..Default::default()
    };
    let game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(120, game_state.borrow_player().hp);
    assert_eq!(9, game_state.borrow_player().attack);
    assert_eq!(15, game_state.borrow_player().defense);

    let guard_ref_1_0 = game_state.borrow_guard_at((1, 0));
    assert_eq!(30, guard_ref_1_0.hp);
    assert_eq!(7, guard_ref_1_0.attack);
    assert_eq!(12, guard_ref_1_0.defense);

    let guard_ref_4_0 = game_state.borrow_guard_at((4, 0));
    assert_eq!(30, guard_ref_4_0.hp);
    assert_eq!(7, guard_ref_4_0.attack);
    assert_eq!(12, guard_ref_4_0.defense);
}

