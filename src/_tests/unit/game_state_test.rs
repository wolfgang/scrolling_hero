use crate::dungeon::helpers::make_dungeon;
use crate::game::GameConfig;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
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

#[test]
fn resolve_combat_damages_both_combatants_and_returns_damages() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);

    // Both player and guard have 0 attack and 0 defense, so every roll is a hit
    let game_config = GameConfig { guard_hp: 20, player_hp: 100, ..Default::default() };
    let mut game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    let player_ref = game_state.player_ref();
    let guard_ref = game_state.guard_ref_at((1, 0));

    assert_eq!(player_ref.borrow().hp, 100);
    assert_eq!(guard_ref.borrow().hp, 20);

    let mut dice_roller = RandomizedDiceRoller::new();
    let (player_result, guard_result) = game_state.resolve_combat((1, 0), &mut dice_roller);

    assert!(player_ref.borrow().hp < 100);
    assert!(guard_ref.borrow().hp < 20);

    assert_eq!(guard_result.damage_done, 100 - player_ref.borrow().hp as u8);
    assert_eq!(player_result.damage_done, 20 - guard_ref.borrow().hp as u8);

}
