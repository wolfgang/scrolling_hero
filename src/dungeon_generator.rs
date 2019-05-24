use rand::Rng;
use rand::rngs::StdRng;

use crate::types::DungeonLayout;

pub struct DungeonGenOpts {
    pub width: usize,
    pub height: usize,
    pub num_paths: u8,
    pub vertical_bias: u8,
    pub horizontal_bias: u8,

}

pub fn dungeon_with_num_paths(gen_opts: &DungeonGenOpts, rng: &mut StdRng) -> DungeonLayout {
    let width = gen_opts.width;
    let height = gen_opts.height;
    let mut dungeon = init_dungeon(width, height);
    for _ in 0..gen_opts.num_paths {
        generate_dungeon_path(&mut dungeon, &gen_opts, rng);
    }

    let exit_position = rng.gen_range(1, width - 2);
    dungeon[height - 1][exit_position] = 'E';

    dungeon
}


fn init_dungeon(width: usize, height: usize) -> DungeonLayout {
    let mut open_row = vec!['.'; width as usize];
    open_row[0] = '#';
    open_row[width as usize - 1] = '#';

    let mut dungeon = vec![vec!['#'; width]; height as usize];
    dungeon[0] = open_row.to_vec();
    dungeon[height - 1] = open_row.to_vec();
    dungeon
}

const LEFT: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;

fn generate_dungeon_path(dungeon: &mut DungeonLayout, gen_opts: &DungeonGenOpts, rng: &mut StdRng) {
    let width = gen_opts.width;
    let height = gen_opts.height;

    let mut x = rng.gen_range(1, width - 2);
    let mut y = 1;
    dungeon[1][x] = '.';

    let mut prev_x = x;

    while y < height - 1 {
        let directions = possible_directions(gen_opts, x, prev_x);
        prev_x = x;

        let index = rng.gen_range(0, directions.len());
        let direction = directions[index];

        match direction {
            LEFT => { x = x - 1; }
            DOWN => { y = y + 1; }
            RIGHT => { x = x + 1; }
            _ => {}
        }

        dungeon[y][x] = '.';
    }
}

fn possible_directions(gen_opts: &DungeonGenOpts, x: usize, prev_x: usize) -> Vec<u8> {
    let mut directions = Vec::new();
    add_direction_times(gen_opts.vertical_bias, DOWN, &mut directions);
    if x - 1 > 1 && x - 1 != prev_x {
        add_direction_times(gen_opts.horizontal_bias, LEFT, &mut directions);
    };
    if x + 1 < gen_opts.width - 1 && x + 1 != prev_x {
        add_direction_times(gen_opts.horizontal_bias, RIGHT, &mut directions);
    };
    directions
}

fn add_direction_times(times: u8, direction: u8, directions: &mut Vec<u8>) {
    for _ in 0..times {
        directions.push(direction);
    }
}
