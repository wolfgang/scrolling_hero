use crate::dungeon::helpers::make_dungeon;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::game::state::GameState;

#[test]
fn construct_with_new() {
    let (dungeon, player_pos) = make_dungeon(vec!["#.@#"]);
    let game_state = GameState::new(dungeon.clone(), player_pos, 20);

    assert_eq!(dungeon, game_state.dungeon);
    assert_eq!(player_pos, game_state.player_position);
    assert_eq!(100, game_state.player_ref().borrow().hp);
}

#[test]
fn resolve_combat() {
    let (dungeon, player_pos) = make_dungeon(vec!["#G@#"]);
    let mut game_state = GameState::new(dungeon.clone(), player_pos, 20);

    assert_eq!(game_state.player_ref().borrow().hp, 100);
    assert_eq!(game_state.guard_ref_at((1, 0)).borrow().hp, 20);


    let mut dice_roller = RandomizedDiceRoller::new();
    game_state.resolve_combat((1, 0), &mut dice_roller);

    assert!(game_state.player_ref().borrow().hp < 100);
    assert!(game_state.guard_ref_at((1, 0)).borrow().hp < 20);
}
