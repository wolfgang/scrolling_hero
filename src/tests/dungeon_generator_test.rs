use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::dungeon_generator::{dungeon_with_num_paths, DungeonGenOpts};
use crate::dungeon_helpers::make_dungeon;
use crate::types::DungeonLayout;

#[test]
fn generates_dungeon_with_one_path() {
    assert_eq!(
        dungeon_layout(vec![
            "#........#",
            "#######..#",
            "####.....#",
            "####...###",
            "######...#",
            "#######..#",
            "#######..#",
            "#......E.#"]),
        single_path_dungeon_from_seed(1000));

    assert_eq!(
        dungeon_layout(vec![
            "#........#",
            "######..##",
            "#####..###",
            "#####..###",
            "##.....###",
            "##.....###",
            "######..##",
            "#.....E..#"]),
        single_path_dungeon_from_seed(2000));

    assert_eq!(
        dungeon_layout(vec![
            "#........#",
            "###.....##",
            "#######..#",
            "###......#",
            "###.######",
            "##..######",
            "##.#######",
            "#.....E..#"]),
        single_path_dungeon_from_seed(3000));
}

#[test]
fn does_not_generate_exit_for_each_path() {
    let mut rng = StdRng::seed_from_u64(1000);
    let dungeon = dungeon_with_num_paths(
        2,
        DungeonGenOpts { vertical_bias: 1, horizontal_bias: 2, width: 10, height: 8 },
        &mut rng);
    assert_eq!(dungeon_layout(vec![
        "#........#",
        "######...#",
        "####.....#",
        "####.....#",
        "####.....#",
        "####.##..#",
        "####.....#",
        "#...E....#"
    ]), dungeon)
}

pub fn single_path_dungeon_from_seed(seed: u64) -> DungeonLayout {
    let mut rng = StdRng::seed_from_u64(seed);
    dungeon_with_num_paths(1, DungeonGenOpts { vertical_bias: 1, horizontal_bias: 2, width: 10, height: 8 }, &mut rng)
}

#[allow(dead_code)]
fn print_dungeon_snapshot(dungeon: &DungeonLayout) {
    for row in dungeon {
        println!("\"{}\",", row.into_iter().collect::<String>());
    }
}

fn dungeon_layout(strings: Vec<&str>) -> DungeonLayout {
    let dungeon = make_dungeon(strings);
    dungeon.0
}
