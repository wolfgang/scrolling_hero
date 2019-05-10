use crate::dungeon_provider::{DungeonProvider, MultiDungeonProvider, SingleDungeonProvider};
use crate::tests::dungeon_helpers::make_dungeon;

#[test]
fn single_dungeon_provider_provides_same_dungeon_every_time() {
    let (dungeon, _) = make_dungeon(vec![
        "#..",
        "##."
    ]);

    let mut provider = SingleDungeonProvider::new(dungeon.clone());

    assert_eq!(dungeon, *provider.next());
    assert_eq!(dungeon, *provider.next());
}

#[test]
fn multi_dungeon_provider_provides_multiple_dungeons() {
    let (dungeon1, _) = make_dungeon(vec!["#.."]);
    let (dungeon2, _) = make_dungeon(vec!["##."]);
    let (dungeon3, _) = make_dungeon(vec!["###"]);

    let mut provider = MultiDungeonProvider::new(vec![dungeon1.clone(), dungeon2.clone(), dungeon3.clone()]);

    assert_eq!(dungeon1, *provider.next());
    assert_eq!(dungeon2, *provider.next());
    assert_eq!(dungeon3, *provider.next());
}