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

    let mut current_key: Option<Key> = None;

    let mut last_action_millis = 0.0;


    while game.is_running() && !rl.window_should_close() {
        current_key = get_current_key_down(&rl, current_key);

        if is_key_released(&rl, KEY_RIGHT, current_key) ||
            is_key_released(&rl, KEY_LEFT, current_key) ||
            is_key_released(&rl, KEY_DOWN, current_key) {
            last_action_millis = 0.0;
            current_key = None;
        }

        let now = rl.get_time();
        if current_key != None && now - last_action_millis > 0.2 {
            last_action_millis = now;
            game.on_key(current_key.unwrap());
        }

        rl.begin_drawing();
        raylib_writer.clear();

        game.render(&mut raylib_writer)?;

        rl.end_drawing();
    }

    Ok(())
}

fn get_current_key_down(rl: &RaylibHandle, current_key: Option<Key>) -> Option<Key> {
    if rl.is_key_down(KEY_RIGHT as i32) {
        return rl_to_term_key(KEY_RIGHT);
    }
    if rl.is_key_down(KEY_LEFT as i32) {
        return rl_to_term_key(KEY_LEFT);
    }

    if rl.is_key_down(KEY_DOWN as i32) {
        return rl_to_term_key(KEY_DOWN);
    }

    current_key
}

fn is_key_released(rl: &RaylibHandle, rl_key: u32, current_key: Option<Key>) -> bool {
    rl.is_key_released(rl_key as i32) && current_key == rl_to_term_key(rl_key)
}

fn rl_to_term_key(rl_key: u32) -> Option<Key> {
    match rl_key {
        KEY_RIGHT => { Some(Key::ArrowRight) }
        KEY_LEFT => { Some(Key::ArrowLeft) }
        KEY_DOWN => { Some(Key::ArrowDown) }
        _ => { None }
    }
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
        let source_rec = Rectangle {
            x: texture_x as f32 * 16.0,
            y: texture_y as f32 * 16.0,
            width: 16.0,
            height: 16.0,
        };

        let scale = 3.0;

        let dest_rec = Rectangle {
            x: self.current_x as f32 * 16.0*scale,
            y: self.current_y as f32 * 16.0*scale,
            width: 16.0*scale,
            height: 16.0*scale
        };

        self.rl.draw_texture_pro(texture, source_rec, dest_rec, Vector2::zero(), 0.0, Color::WHITE);
        //self.rl.draw_texture_rec(texture, rec, position, Color::WHITE);
    }

    fn render_char(&self, b: u8) {
        let str = String::from_utf8(vec![b]).unwrap();
        self.rl.draw_text(str.as_str(), self.current_x * 16, self.current_y * 16, 16, Color::WHITE);
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
