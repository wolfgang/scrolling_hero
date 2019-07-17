use std::io::Write;
use std::rc::Rc;

use console::Key;

use renderer::GameRenderer;
use state::GameState;

use crate::game::dice_roller::DiceRoller;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::types::{DungeonProviderRef, Position};

pub mod renderer;
pub mod state;
pub mod combatant;
pub mod dice_roller;
pub mod randomized_dice_roller;

#[derive(Default, Clone)]
pub struct GameConfig {
    pub camera_offset: i32,
    pub guard_hp: u16,
    pub guard_attack: u8,
    pub guard_defense: u8,

    pub player_hp: u16,
    pub player_attack: u8,
    pub player_defense: u8,

}

impl GameConfig {
    pub fn with_defaults() -> GameConfig {
        GameConfig { camera_offset: 1, ..Default::default() }
    }
}

pub struct Game {
    pub game_state: GameState,
    dungeon_provider: DungeonProviderRef,
    is_running: bool,
    game_renderer: GameRenderer,
    dice_roller: Box<dyn DiceRoller>,
    hud: Vec<String>,
    config: GameConfig,
    guard_in_combat: Option<Position>,
}

impl Game {
    pub fn with_config(config: &GameConfig, dungeon_provider_ref: &DungeonProviderRef) -> Game {
        let dungeon_provider = Rc::clone(dungeon_provider_ref);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();

        let mut game = Game {
            game_state: GameState::from_game_config(config, dungeon, player_position),
            dungeon_provider,
            game_renderer: GameRenderer::new(config.camera_offset),
            dice_roller: Box::from(RandomizedDiceRoller::new()),
            is_running: true,
            hud: Vec::with_capacity(10),
            config: (*config).clone(),
            guard_in_combat: None,
        };

        game.reset_hud();

        game
    }

    pub fn override_dice_roller(&mut self, dice_roller: Box<dyn DiceRoller>) {
        self.dice_roller = dice_roller;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn render(&mut self, writer: &mut Write) -> std::io::Result<(u32)> {
        self.game_renderer.render(
            writer,
            &self.game_state,
            &self.hud,
        )
    }

    pub fn on_key(&mut self, key: Key) {
        match key {
            Key::ArrowLeft => {
                self.process_neighbor(-1, 0);
                self.game_state.attempt_player_move_to(-1, 0);
            }
            Key::ArrowRight => {
                self.process_neighbor(1, 0);
                self.game_state.attempt_player_move_to(1, 0);
            }
            Key::ArrowDown => {
                self.process_neighbor(0, 1);
                self.game_state.attempt_player_move_to(0, 1);
            }
            Key::Escape => {
                self.is_running = false;
            }
            _ => {}
        }

        if self.under_player() == 'H' {
            let heal = self.game_state.heal_player(&mut *self.dice_roller);
            self.reset_hud();
            self.hud.push(String::from(format!("Player regains {} HP", heal)));
        }

        if self.under_player() == 'E' { self.goto_next_dungeon(); }
    }

    fn under_player(&self) -> char {
        self.game_state.neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: u32) {
        match self.game_state.neighbor_at(x_offset, y_offset) {
            Some((pos, tile)) => {
                if tile == 'G' {
                    self.guard_in_combat = Some(pos);
                    let (damage_to_guard, damage_to_player) = self.game_state.resolve_combat(pos, &mut *self.dice_roller);
                    self.reset_hud();
                    self.show_combat_messages(pos, damage_to_guard, damage_to_player);
                    let guard_hp = self.game_state.borrow_guard_at(pos).hp;
                    if guard_hp <= 0 { self.guard_in_combat = None; }
                    if self.player_hp() <= 0 { self.is_running = false; }
                } else {
                    self.reset_hud();
                    if self.guard_in_combat != None {
                        let guard_pos = self.guard_in_combat.unwrap();
                        let guard_ref = self.game_state.guard_ref_at(guard_pos);
                        let player_ref = self.game_state.player_ref();
                        let result = guard_ref.borrow().attack(&player_ref, &mut *self.dice_roller);
                        self.hud.push(Game::attack_message("Guard", "Player", result, 100));
                        self.guard_in_combat = None;
                    }
                }
            }

            None => {}
        }
    }

    fn reset_hud(&mut self) {
        self.hud.clear();
        self.hud.push(Game::player_health_message(self.player_hp()));
    }

    fn show_combat_messages(&mut self, guard_pos: (u32, u32), guard_result: (u8, bool), player_result: (u8, bool)) {
        let guard_health = self.game_state.borrow_guard_at(guard_pos).hp;
        let player_health = self.player_hp();
        self.hud.push(Game::attack_message("Player", "Guard", guard_result, player_health));
        self.hud.push(Game::attack_message("Guard", "Player", player_result, guard_health));
    }

    fn player_hp(&self) -> i16 {
        self.game_state.borrow_player().hp
    }

    fn attack_message(attacker: &str, target: &str, combat_result: (u8, bool), attacker_hp: i16) -> String {
        if attacker_hp <= 0 {
            return String::from(format!("{} dies!", attacker));
        }

        let (damage, is_crit) = combat_result;

        if damage > 0 {
            let action = if is_crit { "CRITS" } else { "hits" };
            return String::from(format!("{} {} {} for {}", attacker, action, target, damage));
        }
        String::from(format!("{} misses {}!", attacker, target))
    }

    fn player_health_message(player_hp: i16) -> String {
        format!("HP: {}", player_hp)
    }

    fn goto_next_dungeon(&mut self) {
        match self.dungeon_provider.borrow_mut().next() {
            Some((next_dungeon, next_player_pos)) => {
                let player_hp = self.player_hp();
                self.game_state = GameState::from_game_config(&self.config, next_dungeon, next_player_pos);
                self.game_state.player_ref().borrow_mut().hp = player_hp;
            }
            None => { self.is_running = false; }
        }
    }
}
