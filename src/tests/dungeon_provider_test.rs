use crate::dungeon_provider::SingleDungeonProvider;
use crate::tests::dungeon_helpers::make_dungeon;

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

//#[test]
//fn multi_dungeon_provider_provides_multiple_dungeons() {
//    let (dungeon1, _) = make_dungeon(vec!["#.."]);
//    let (dungeon2, _) = make_dungeon(vec!["##."]);
//    let (dungeon3, _) = make_dungeon(vec!["###"]);
//
//    let mut provider = MultiDungeonProvider::new(vec![dungeon1.clone(), dungeon2.clone(), dungeon3.clone()]);
//
//    assert_eq!(dungeon1, provider.next().unwrap());
//    assert_eq!(dungeon2, provider.next().unwrap());
//    assert_eq!(dungeon3, provider.next().unwrap());
//}
//
//#[test]
//fn multi_dungeon_provider_can_be_constructed_as_rc_refcell() {
//    let (dungeon1, _) = make_dungeon(vec!["#.."]);
//
//    let provider = MultiDungeonProvider::shared(vec![dungeon1.clone()]);
//    assert_eq!(dungeon1, provider.borrow_mut().next().unwrap());
//}