use crate::dungeon::helpers::make_dungeon;
use crate::dungeon::provider::*;

#[test]
fn single_dungeon_provider_provides_same_dungeon_every_time() {
    let (dungeon, player_pos) = make_dungeon(vec![
        "#@.",
        "##."
    ]);

    let mut provider = SingleDungeonProvider::new(dungeon.clone(), player_pos);

    assert_eq!((dungeon.clone(), player_pos), provider.next().unwrap());
    assert_eq!((dungeon.clone(), player_pos), provider.next().unwrap());
}

#[test]
fn multi_dungeon_provider_provides_multiple_dungeons() {
    let (dungeon1, player_pos1) = make_dungeon(vec!["#.."]);
    let (dungeon2, player_pos2) = make_dungeon(vec!["##."]);
    let (dungeon3, player_pos3) = make_dungeon(vec!["###"]);

    let mut provider = MultiDungeonProvider::new(vec![
        (dungeon1.clone(), player_pos1),
        (dungeon2.clone(), player_pos2),
        (dungeon3.clone(), player_pos3)]);

    assert_eq!((dungeon1, player_pos1), provider.next().unwrap());
    assert_eq!((dungeon2, player_pos2), provider.next().unwrap());
    assert_eq!((dungeon3, player_pos3), provider.next().unwrap());
}

#[test]
fn multi_dungeon_provider_can_be_constructed_as_rc_refcell() {
    let (dungeon1, player_pos1) = make_dungeon(vec!["#.."]);

    let provider = MultiDungeonProvider::shared(vec![(dungeon1.clone(), player_pos1)]);
    assert_eq!((dungeon1, player_pos1), provider.borrow_mut().next().unwrap());
}

#[test]
fn multi_dungeon_provider_returns_none_after_last_dungeon() {
    let (dungeon1, player_pos1) = make_dungeon(vec!["#.."]);

    let mut provider = MultiDungeonProvider::new(vec![(dungeon1.clone(), player_pos1)]);

    assert_eq!((dungeon1, player_pos1), provider.next().unwrap());
    assert_eq!(None, provider.next());
}
