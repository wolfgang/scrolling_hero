use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::dungeon::decorator;
use crate::tests::dungeon_test_helpers::*;

#[test]
fn generates_guards_randomly_on_floor() {
    let dungeon = dungeon_layout(vec![
        "#.....#",
        "###...#",
        "#...#.#",
        "#.#...#",
        "#.....#",
    ]);

    let mut dungeon1 = dungeon.clone();
    let mut dungeon2 = dungeon.clone();
    decorator::add_guards(&mut dungeon1, 30, &mut StdRng::seed_from_u64(1000));
    decorator::add_guards(&mut dungeon2, 50, &mut StdRng::seed_from_u64(2000));

    assert_eq!(
        dungeon_layout(vec![
            "#.....#",
            "###...#",
            "#...#G#",
            "#.#GG.#",
            "#.....#",
        ]),
        dungeon1);

    assert_eq!(
        dungeon_layout(vec![
            "#.....#",
            "###.G.#",
            "#.G.#.#",
            "#.#G..#",
            "#.....#",
        ]),
        dungeon2
    );
}
