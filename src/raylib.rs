use std::io::{Error, Write};

use console::Key;
use raylib::*;
use raylib::consts::*;

use crate::game::Game;

pub fn run_game_in_raylib(game: &mut Game, dungeon_width: usize) -> std::io::Result<()> {
    let rl = raylib::init()
        .size(1200, 768)
        .title("Scrolling Hero")
        .build();

    let mut raylib_writer = RaylibWriter::new(&rl, dungeon_width);

    while game.is_running() && !rl.window_should_close() {
        rl.begin_drawing();
        raylib_writer.clear();

        game.render(&mut raylib_writer)?;

        if rl.is_key_pressed(KEY_RIGHT as i32) {
            game.on_key(Key::ArrowRight);
        }
        if rl.is_key_pressed(KEY_LEFT as i32) {
            game.on_key(Key::ArrowLeft);
        }

        if rl.is_key_pressed(KEY_DOWN as i32) {
            game.on_key(Key::ArrowDown);
        }

        rl.end_drawing();
    }

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

    fn render_on_floor(&self, texture_x: u8, texture_y: u8, texture: &Texture2D, key: u8) {
        if self.current_x < self.dungeon_width as i32 {
            self.render_tile(0, 5, &self.dungeon_textures);
            self.render_tile(texture_x, texture_y, texture);
        } else {
            self.render_char(key);
        }
    }

    fn render_tile(&self, texture_x: u8, texture_y: u8, texture: &Texture2D) {
        let rec = Rectangle {
            x: texture_x as f32 * 16.0,
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

    fn render_char(&self, b: u8) {
        let str = String::from_utf8(vec![b]).unwrap();
        self.rl.draw_text(str.as_str(), self.current_x * 16, self.current_y * 16, 12, Color::GREEN);
    }

    pub fn clear(&mut self) {
        self.current_y = 0;
        self.rl.clear_background(Color::BLACK);
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
                    self.render_on_floor(0, 5, &self.player_textures, *b);
                }

                'G' => {
                    self.render_on_floor(0, 2, &self.monster_textures, *b);
                }

                'H' => {
                    self.render_on_floor(0, 2, &self.potion_textures, *b);
                }

                'E' => {
                    self.render_on_floor(2, 4, &self.dungeon_textures, *b);
                }

                _ => {
                    self.render_char(*b);
                }
            }

            self.current_x += 1;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
