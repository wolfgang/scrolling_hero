use rand::Rng;
use rand::rngs::StdRng;

use crate::types::DungeonLayout;

pub struct DungeonGenOpts {
    pub width: usize,
    pub height: usize,
    pub vertical_bias: u16,
    pub horizontal_bias: u16,

}

pub fn dungeon_with_num_paths(
    num_paths: u16,
    gen_opts: DungeonGenOpts,
    rng: &mut StdRng) -> DungeonLayout {
    let width = gen_opts.width;
    let height = gen_opts.height;
    let mut dungeon = init_dungeon(width, height);
    for _ in 0..num_paths {
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
        let mut directions = Vec::new();
        for _ in 0..gen_opts.vertical_bias {
            directions.push(DOWN);
        }
        if x - 1 > 1 && x - 1 != prev_x {
            for _ in 0..gen_opts.horizontal_bias {
                directions.push(LEFT);
            }
        };
        if x + 1 < width - 1 && x + 1 != prev_x {
            for _ in 0..gen_opts.horizontal_bias {
                directions.push(RIGHT);
            }

        };

        prev_x = x;

        let index = rng.gen_range(0, directions.len());
        let direction = directions[index];

        match direction {
            LEFT => {
                x = x - 1;
            }
            DOWN => {
                y = y + 1;
            }
            RIGHT => {
                x = x + 1;
            }

            _ => {}
        }

        dungeon[y][x] = '.';
    }
}
