use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use console::Key;

use renderer::GameRenderer;
use state::GameState;

use crate::game::combatant::CombatResult;
use crate::game::dice_roller::DiceRoller;
use crate::game::randomized_dice_roller::RandomizedDiceRoller;
use crate::types::{CombatantRef, DungeonProviderRef};

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
        GameConfig { camera_offset: 1, player_hp: 100, ..Default::default() }
    }
}

pub struct Game {
    game_state: GameState,
    dungeon_provider: DungeonProviderRef,
    is_running: bool,
    game_renderer: GameRenderer,
    dice_roller: Box<dyn DiceRoller>,
    hud: Vec<String>,
    config: GameConfig,
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

    pub fn render(&mut self, writer: &mut dyn Write) -> std::io::Result<(u32)> {
        self.game_renderer.render(
            writer,
            &self.game_state,
            &self.hud,
        )
    }

    pub fn on_key(&mut self, key: Key) {
        let hud_messages: RefCell<Vec<String>> = RefCell::new(Vec::with_capacity(2));

        match key {
            Key::ArrowLeft => {
                self.process_neighbor(-1, 0, &hud_messages);
            }
            Key::ArrowRight => {
                self.process_neighbor(1, 0, &hud_messages);
            }
            Key::ArrowDown => {
                self.process_neighbor(0, 1, &hud_messages);
            }
            Key::Escape => {
                self.is_running = false;
            }
            _ => {}
        }

        if self.under_player() == 'H' {
            let heal = self.game_state.heal_player(&mut *self.dice_roller);
            hud_messages.borrow_mut().push(String::from(format!("Player regains {} HP", heal)));
        }

        if self.under_player() == 'E' { self.goto_next_dungeon(); }

        self.reset_hud();
        self.hud.append(hud_messages.borrow_mut().as_mut());
    }

    pub fn get_player_hp(&self) -> i16 {
        self.game_state.borrow_player().hp
    }

    pub fn get_player_ref(&self) -> CombatantRef {
        self.game_state.player_ref()
    }

    fn under_player(&self) -> char {
        self.game_state.neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: u32, hud_messages: &RefCell<Vec<String>>) {

        self.game_state.process_move_to(
            x_offset,
            y_offset,
            &mut *self.dice_roller,
            |player_result, guard_result| {
                hud_messages.borrow_mut().push(Game::player_combat_message(player_result));
                hud_messages.borrow_mut().push(Game::guard_combat_message(guard_result));
            },
            |result| {
                hud_messages.borrow_mut().push(Game::guard_combat_message(result));
            });

        if self.get_player_hp() <= 0 { self.is_running = false; }
    }

    fn reset_hud(&mut self) {
        self.hud.clear();
        self.hud.push(Game::player_health_message(self.get_player_hp()));
    }

    fn player_combat_message(combat_result: CombatResult) -> String {
        Game::attack_message("Player", "Guard", combat_result)
    }

    fn guard_combat_message(combat_result: CombatResult) -> String {
        Game::attack_message("Guard", "Player", combat_result)
    }

    fn attack_message(attacker: &str, target: &str, combat_result: CombatResult) -> String {
        if combat_result.attacker_dead {
            return String::from(format!("{} dies!", attacker));
        }

        if combat_result.damage_done > 0 {
            let action = if combat_result.is_crit { "CRITS" } else { "hits" };
            return String::from(
                format!("{} {} {} for {}", attacker, action, target, combat_result.damage_done)
            );
        }
        String::from(format!("{} misses {}!", attacker, target))
    }

    fn player_health_message(player_hp: i16) -> String {
        format!("HP: {}", player_hp)
    }

    fn goto_next_dungeon(&mut self) {
        match self.dungeon_provider.borrow_mut().next() {
            Some((next_dungeon, next_player_pos)) => {
                let current_player_hp = self.get_player_hp();
                self.game_state = GameState::from_game_config(&self.config, next_dungeon, next_player_pos);
                self.game_state.reset_player_hp(current_player_hp);
            }
            None => { self.is_running = false; }
        }
    }
}
