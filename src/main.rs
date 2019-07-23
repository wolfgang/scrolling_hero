use std::io::{Error, Write};
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

    for _ in 1..50 {
        dungeons.push(generate_dungeon(60, 40, &mut rng));
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

//    let rl = raylib::init()
//        .size(800, 450)
//        .title("Texture Test")
//        .build();
//
//    let mut raylib_writer = RaylibWriter::new(&rl);
//
//    while game.is_running() && !rl.window_should_close() {
//        rl.begin_drawing();
//        rl.clear_background(Color::BLACK);
//        raylib_writer.clear();
//
//        let num_lines = game.render(&mut raylib_writer)?;
//
//        if rl.is_key_pressed(KEY_RIGHT as i32) {
//            game.on_key(Key::ArrowRight);
//        }
//        if rl.is_key_pressed(KEY_LEFT as i32) {
//            game.on_key(Key::ArrowLeft);
//        }
//
//        if rl.is_key_pressed(KEY_DOWN as i32) {
//            game.on_key(Key::ArrowDown);
//        }
//
//
////        game.on_key(term.read_key().unwrap());
////        term.clear_last_lines(num_lines as usize)?;
//
//        rl.end_drawing();
//    }

    while game.is_running() {
        let num_lines = game.render(&mut term)?;
        game.on_key(term.read_key().unwrap());
        term.clear_last_lines(num_lines as usize)?;
    }

    term.write_line("Thanks for playing!")?;

    Ok(())
}

struct RaylibWriter<'a> {
    current_x: i32,
    current_y: i32,
    rl: &'a RaylibHandle,
}

impl<'a> RaylibWriter<'a> {
    pub fn new(rl: &'a RaylibHandle) -> RaylibWriter {
        RaylibWriter {
            rl,
            current_x: 0,
            current_y: 0,
        }
    }

    pub fn clear(&mut self) {
        self.current_y = 0;
    }
}

impl Write for RaylibWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let mut str = String::with_capacity(100);
        let mut written = 0;
        for b in buf {
            written += 1;
            if *b == '\n' as u8 {
                self.rl.draw_text(str.as_str(), 0, self.current_y * 16, 16, Color::GOLD);
                self.current_x = 0;
                self.current_y += 1;
                str.clear();
            } else {
                str.push(*b as char);
            }
        }


        Ok(written)
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
