use crate::dungeon::helpers::make_dungeon;
use crate::game::GameConfig;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::game::state::GameState;

#[test]
fn construct_with_from_game_config() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);
    let game_config = GameConfig { camera_offset: 1, guard_hp: 30 };
    let game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player_ref().borrow().hp);
    assert_eq!(30, game_state.borrow_guard_at((1, 0)).hp);

}

#[test]
fn resolve_combat() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);
    let game_config = GameConfig { camera_offset: 1, guard_hp: 20 };

    let mut game_state = GameState::from_game_config(&game_config, dungeon.clone(), player_pos);

    assert_eq!(game_state.player_ref().borrow().hp, 100);
    assert_eq!(game_state.borrow_guard_at((1, 0)).hp, 20);


    let mut dice_roller = RandomizedDiceRoller::new();
    game_state.resolve_combat((1, 0), &mut dice_roller);

    assert!(game_state.player_ref().borrow().hp < 100);
    assert!(game_state.borrow_guard_at((1, 0)).hp < 20);
}
