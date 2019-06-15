use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::dungeon::generator::{dungeon_with_num_paths, DungeonGenOpts};
use crate::tests::dungeon_test_helpers::*;
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
    let gen_opts = DungeonGenOpts {
        width: 10,
        height: 8,
        num_paths: 2,
        vertical_bias: 1,
        horizontal_bias: 2,
    };
    let dungeon = dungeon_with_num_paths(&gen_opts, &mut rng);
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
    let gen_opts = DungeonGenOpts {
        width: 10,
        height: 8,
        num_paths: 1,
        vertical_bias: 1,
        horizontal_bias: 2,
    };
    dungeon_with_num_paths(&gen_opts, &mut rng)
}
