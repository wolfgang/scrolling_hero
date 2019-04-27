use rand::{thread_rng, Rng};




fn generate_wall(length: usize) -> String {
    let mut result = String::from("");
    let mut rng = thread_rng();

    let hole_position = rng.gen_range(1, length-1);

    for i in 0 .. length {
        if i == hole_position { result.push('.') }
        else { result.push('#') }
    }


    result

}

#[test]
fn generate_wall_with_one_hole_not_at_edge() {
    let wall1 = generate_wall(100);
    let wall2 = generate_wall(100);
    let wall3 = generate_wall(100);
    assert_eq!(100, wall1.len());
    assert_eq!(100, wall2.len());
    assert_eq!(100, wall3.len());
    assert!(wall1.contains("#.#"));
    assert!(wall2.contains("#.#"));
    assert!(wall3.contains("#.#"));
    assert_single_hole_in(&wall1);
    assert_single_hole_in(&wall2);
    assert_single_hole_in(&wall3);
}

#[test]
fn subsequent_walls_have_different_hole_postion() {
    let wall1 = generate_wall(100);
    let wall2 = generate_wall(100);
    assert_ne!(hole_index(&wall1), hole_index(&wall2));
}

fn assert_single_hole_in(wall: &str) {
    assert_eq!(2, wall.split('.').collect::<Vec<&str>>().len());
}

fn hole_index(wall: &str) -> usize {
    wall.find('.').unwrap()
}
