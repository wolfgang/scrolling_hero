use std::cmp::{max, min};

use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

pub fn generate_dungeon(length: u32) -> Vec<String> {
    let mut rng = thread_rng();

    let entrance_x = rng.gen_range(1, length - 1);

    let mut line1 = String::with_capacity(length as usize);
    for i in 0..length {
        if i == entrance_x { line1.push('.') } else { line1.push('#') }
    }

    let line2 = generate_row(&line1, entrance_x, &mut rng);
    let line3 = generate_row(&line1, entrance_x, &mut rng);
    let line4 = generate_row(&line1, entrance_x, &mut rng);
    let line5 = generate_row(&line1, entrance_x, &mut rng);

    vec![line1, line2, line3, line4, line5]
}

fn generate_row(first_row: &String, entrance_x: u32, rng: &mut ThreadRng) -> String {
    let mut row = first_row.clone();

    let offset1 = rng.gen_range(0, 5);
    let offset2 = rng.gen_range(0, 5);

    let from = max(1, entrance_x as i32 - offset1) as usize;
    let to = min(first_row.len() as u32 - 1, entrance_x + offset2) as usize;

    row.replace_range(from..to, ".".repeat(to - from + 1).as_str());
    row
}
