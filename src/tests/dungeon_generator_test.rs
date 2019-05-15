use rand::{Rng, thread_rng};

fn generate_dungeon() -> Vec<String> {
    let mut rng = thread_rng();

    let length = 10;
    let hole_position = rng.gen_range(1, length - 1);

    let mut result = String::with_capacity(length);
    for i in 0..length {
        if i == hole_position { result.push('.') } else { result.push('#') }
    }

    vec![result]
}


#[test]
fn first_line_contains_one_random_hole() {
    let dungeon = generate_dungeon();
    assert!(dungeon[0].contains("#.#"));
    assert_eq!(1, dungeon[0].matches(".").count());
}

#[test]
fn two_dungeon_have_different_entrance_hole_position() {
    let dungeon1 = generate_dungeon();
    let dungeon2 = generate_dungeon();
    assert_ne!(dungeon1[0].find('.').unwrap(), dungeon2[0].find('.').unwrap());
}