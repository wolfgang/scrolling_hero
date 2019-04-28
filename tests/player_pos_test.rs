use sch::player_pos;

#[test]
fn new_creates_rc_with_refcell() {
    let player_pos = player_pos::new(1, 2);

    assert_eq!(1, player_pos.borrow().0);
    assert_eq!(2, player_pos.borrow().1);

    player_pos.borrow_mut().0 += 10;
    player_pos.borrow_mut().1 += 10;

    assert_eq!(11, player_pos.borrow().0);
    assert_eq!(12, player_pos.borrow().1);
}