use crate::dungeon::helpers::make_dungeon;
use crate::game::GameConfig;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::game::state::GameState;

#[test]
fn construct_with_from_game_config() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);
    let game_config = GameConfig {
        guard_hp: 30,
        guard_attack: 7,
        guard_defense: 12,
        ..Default::default()
    };
    let game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.borrow_player().hp);
    let guard_ref = game_state.borrow_guard_at((1, 0));
    assert_eq!(30, guard_ref.hp);
    assert_eq!(7, guard_ref.attack);
    assert_eq!(12, guard_ref.defense);

}

#[test]
fn resolve_combat_with_zero_attack_or_defense() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);
    let game_config = GameConfig { guard_hp: 20, ..Default::default() };

    let mut game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    assert_eq!(game_state.borrow_player().hp, 100);
    assert_eq!(game_state.borrow_guard_at((1, 0)).hp, 20);


    let mut dice_roller = RandomizedDiceRoller::new();
    game_state.resolve_combat((1, 0), &mut dice_roller);

    assert!(game_state.borrow_player().hp < 100);
    assert!(game_state.borrow_guard_at((1, 0)).hp < 20);
}
