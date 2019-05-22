use std::collections::HashMap;

use crate::dungeon_generator::create_dungeon_with_one_path;
use crate::dungeon_helpers::make_dungeon;
use crate::types::DungeonLayout;

#[test]
fn generates_dungeon_with_one_path() {
    let dungeon1 = create_dungeon_with_one_path(1000);
    let dungeon2 = create_dungeon_with_one_path(2000);
    let dungeon3 = create_dungeon_with_one_path(3000);

    let ref_dungeons = seeds_to_dungeons();

    assert_eq!(ref_dungeons.get(&1000), Some(&dungeon1));
    assert_eq!(ref_dungeons.get(&2000), Some(&dungeon2));
    assert_eq!(ref_dungeons.get(&3000), Some(&dungeon3));
}

#[allow(dead_code)]
fn print_dungeon_snapshot(dungeon: &DungeonLayout) {
    for row in dungeon {
        println!("\"{}\",", row.into_iter().collect::<String>());
    }
}

fn dungeon_from(strings: Vec<&str>) -> DungeonLayout {
    let dungeon = make_dungeon(strings);
    dungeon.0
}

fn seeds_to_dungeons() -> HashMap<u64, DungeonLayout> {
    let mut hm = HashMap::new();

    hm.insert(1000, dungeon_from(vec![
        "#........#",
        "#######..#",
        "####.....#",
        "####...###",
        "######...#",
        "#######..#",
        "#######..#",
        "#........#"]));


    hm.insert(2000, dungeon_from(vec![
        "#........#",
        "######..##",
        "#####..###",
        "#####..###",
        "##.....###",
        "##.....###",
        "######..##",
        "#........#"]));

    hm.insert(3000, dungeon_from(vec![
        "#........#",
        "###.....##",
        "#######..#",
        "###......#",
        "###.######",
        "##..######",
        "##.#######",
        "#........#"]));
    hm
}