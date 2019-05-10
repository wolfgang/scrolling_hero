use crate::dungeon_provider::{DungeonProvider, SingleDungeonProvider};
use crate::tests::dungeon_helpers::make_dungeon;

#[test]
fn single_dungeon_provider_provides_same_dungeon_every_time() {
    let (dungeon, _) = make_dungeon(vec![
        "#..",
        "##."
    ]);

    let provider = SingleDungeonProvider::new(dungeon.clone());

    assert_eq!(dungeon, *provider.next());
    assert_eq!(dungeon, *provider.next());
}