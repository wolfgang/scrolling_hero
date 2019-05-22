use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::dungeon_generator::dungeon_with_num_paths;
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
            "#........#"]),
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
            "#........#"]),
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
            "#........#"]),
        single_path_dungeon_from_seed(3000));
}

pub fn single_path_dungeon_from_seed(seed: u64) -> DungeonLayout {
    let mut rng = StdRng::seed_from_u64(seed);
    dungeon_with_num_paths(1, 10, 8, &mut rng)
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
