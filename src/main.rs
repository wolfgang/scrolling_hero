use std::borrow::Borrow;
use std::io::{Error, Write};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use console::Key;
use console::Term;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use raylib::*;
use raylib::consts::{KEY_DOWN, KEY_LEFT, KEY_RIGHT};

use sch::dungeon::decorator;
use sch::dungeon::generator::{dungeon_with_num_paths, DungeonGenOpts};
use sch::dungeon::provider::MultiDungeonProvider;
use sch::game::{Game, GameConfig};
use sch::types::DungeonDefinition;

fn main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut dungeons = Vec::new();

    let dungeon_width = 40;

    for _ in 1..50 {
        dungeons.push(generate_dungeon(dungeon_width, 40, &mut rng));
    }

    let dungeon_provider = MultiDungeonProvider::shared(dungeons);

    let game_config = GameConfig {
        camera_offset: 2,
        guard_hp: 20,
        guard_attack: 2,
        guard_defense: 10,
        player_hp: 100,
        player_attack: 5,
        player_defense: 15,
    };

    let mut game = Game::with_config(&game_config, &dungeon_provider);

    let rl = raylib::init()
        .size(1024, 768)
        .title("Texture Test")
        .build();

    let mut raylib_writer = RaylibWriter::new(&rl, dungeon_width);

    while game.is_running() && !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        raylib_writer.clear();

        let num_lines = game.render(&mut raylib_writer)?;

        if rl.is_key_pressed(KEY_RIGHT as i32) {
            game.on_key(Key::ArrowRight);
        }
        if rl.is_key_pressed(KEY_LEFT as i32) {
            game.on_key(Key::ArrowLeft);
        }

        if rl.is_key_pressed(KEY_DOWN as i32) {
            game.on_key(Key::ArrowDown);
        }


//        game.on_key(term.read_key().unwrap());
//        term.clear_last_lines(num_lines as usize)?;

        rl.end_drawing();
    }

//    while game.is_running() {
//        let num_lines = game.render(&mut term)?;
//        game.on_key(term.read_key().unwrap());
//        term.clear_last_lines(num_lines as usize)?;
//    }
//
//    term.write_line("Thanks for playing!")?;

    Ok(())
}

struct RaylibWriter<'a> {
    rl: &'a RaylibHandle,
    dungeon_width: usize,
    player_textures: Texture2D,
    dungeon_textures: Texture2D,
    monster_textures: Texture2D,
    potion_textures: Texture2D,
    current_x: i32,
    current_y: i32,
}

impl<'a> RaylibWriter<'a> {
    pub fn new(rl: &'a RaylibHandle, dungeon_width: usize) -> RaylibWriter {
        RaylibWriter {
            rl,
            dungeon_width,
            player_textures: rl.load_texture("resources/players.png"),
            monster_textures: rl.load_texture("resources/monsters.png"),
            dungeon_textures: rl.load_texture("resources/stone_walls.png"),
            potion_textures: rl.load_texture("resources/fireball.png"),
            current_x: 0,
            current_y: 0,
        }
    }

    fn render_tile(&self, texture_x: u8, texture_y: u8, texture: &Texture2D) {
        let rec = Rectangle {
            x: texture_x as f32,
            y: texture_y as f32 * 16.0,
            width: 16.0,
            height: 16.0,
        };
        let position = Vector2 {
            x: self.current_x as f32 * 16.0,
            y: self.current_y as f32 * 16.0,
        };
        self.rl.draw_texture_rec(texture, rec, position, Color::WHITE);
    }

    pub fn clear(&mut self) {
        self.current_y = 0;
    }
}

impl Write for RaylibWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for b in buf {
            let key = *b as char;
            match key {
                '\n' => {
                    self.current_x = 0;
                    self.current_y += 1;
                }
                '#' => {
                    self.render_tile(0, 1, &self.dungeon_textures);
                }

                '.' => {
                    self.render_tile(0, 5, &self.dungeon_textures);
                }

                '@' => {
                    self.render_tile(0, 5, &self.dungeon_textures);
                    self.render_tile(0, 5, &self.player_textures);
                }

                'G' => {
                    if self.current_x < self.dungeon_width as i32 {
                        self.render_tile(0, 5, &self.dungeon_textures);
                        self.render_tile(0, 2, &self.monster_textures);
                    }
                }

                'H' => {
                    if self.current_x < self.dungeon_width as i32 {
                        self.render_tile(0, 5, &self.dungeon_textures);
                        self.render_tile(0, 2, &self.potion_textures);
                    }
                }

                _ => {}
            }

            self.current_x += 1;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

fn generate_dungeon(width: usize, height: usize, rng: &mut StdRng) -> DungeonDefinition {
    let opts = DungeonGenOpts {
        width,
        height,
        num_paths: (width / 10) as u8,
        vertical_bias: 1,
        horizontal_bias: 2,
    };

    let mut dungeon = dungeon_with_num_paths(&opts, rng);
    decorator::add_guards(&mut dungeon, 10, rng);
    decorator::add_health_potions(&mut dungeon, 2, rng);

    let width = dungeon[0].len();
    let player_position = rng.gen_range(1, width as u32 - 2);

    (dungeon, (player_position, 0))
}
