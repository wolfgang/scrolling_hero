use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use console::Key;

use hud::Hud;
use renderer::GameRenderer;
use state::GameState;

use crate::game::combatant::CombatResult;
use crate::types::{CombatantRef, DiceRollerRef, DungeonProviderRef};

pub mod renderer;
pub mod hud;
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
    game_state: RefCell<GameState>,
    dungeon_provider: DungeonProviderRef,
    is_running: bool,
    game_renderer: GameRenderer,
    hud_lines: Vec<String>,
    hud: Hud,
    config: GameConfig,
    combat_log: RefCell<Vec<String>>

}

impl Game {
    pub fn with_config(config: &GameConfig, dungeon_provider_ref: &DungeonProviderRef) -> Game {
        let dungeon_provider = Rc::clone(dungeon_provider_ref);

        let (dungeon, player_position) = dungeon_provider.borrow_mut().next().unwrap();

        let mut game = Game {
            game_state: GameState::from_config_as_ref(config, dungeon, player_position),
            dungeon_provider,
            game_renderer: GameRenderer::new(config.camera_offset),
            is_running: true,
            hud_lines: Vec::with_capacity(10),
            hud: Hud::new(),
            combat_log: RefCell::new(Vec::with_capacity(2)),
            config: (*config).clone(),
        };

        game.refresh_hud();

        game
    }

    pub fn override_dice_roller(&mut self, dice_roller: DiceRollerRef) {
        self.game_state.borrow_mut().override_dice_roller(dice_roller)
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn render(&mut self, writer: &mut dyn Write) -> std::io::Result<(u32)> {
        self.game_renderer.render(
            writer,
            &self.game_state.borrow(),
            &self.hud_lines,
        )
    }

    pub fn on_key(&mut self, key: Key) {

        match key {
            Key::ArrowLeft => {
                self.process_neighbor(-1, 0);
            }
            Key::ArrowRight => {
                self.process_neighbor(1, 0);
            }
            Key::ArrowDown => {
                self.process_neighbor(0, 1);
            }
            Key::Escape => {
                self.is_running = false;
            }
            _ => {}
        }

        if self.under_player() == 'H' {
            let heal = self.game_state.borrow_mut().heal_player();
            self.add_combat_log(String::from(format!("Player regains {} HP", heal)));
        }

        if self.under_player() == 'E' { self.goto_next_dungeon(); }

        self.refresh_hud();
    }

    pub fn get_player_hp(&self) -> i16 {
        self.game_state.borrow().borrow_player().hp
    }

    pub fn get_player_ref(&self) -> CombatantRef {
        self.game_state.borrow().player_ref()
    }

    fn under_player(&self) -> char {
        self.game_state.borrow().neighbor_at(0, 0).unwrap().1
    }

    fn process_neighbor(&mut self, x_offset: i32, y_offset: u32) {
        self.game_state.borrow_mut().process_move_to(
            x_offset,
            y_offset,
            |player_result, guard_result| {
                self.add_combat_log(Hud::player_combat_message(player_result));
                self.add_combat_log(Hud::guard_combat_message(guard_result));
            },
            |result| {
                self.add_combat_log(Hud::guard_combat_message(result));
            });

        if self.get_player_hp() <= 0 { self.is_running = false; }
    }

    fn refresh_hud(&mut self) {
        self.hud_lines.clear();
        self.hud_lines.push(Hud::player_health_message(self.get_player_hp()));
        self.hud_lines.append(self.combat_log.borrow_mut().as_mut());
    }

    fn add_combat_log(&self, message: String) {
        self.combat_log.borrow_mut().push(message);
    }

    fn goto_next_dungeon(&mut self) {
        match self.dungeon_provider.borrow_mut().next() {
            Some((next_dungeon, next_player_pos)) => {
                let current_player_hp = self.get_player_hp();
                self.game_state = GameState::from_config_as_ref(&self.config, next_dungeon, next_player_pos);
                self.game_state.borrow_mut().reset_player_hp(current_player_hp);
            }
            None => { self.is_running = false; }
        }
    }
}
