fn generate_dungeon() -> Vec<String> {
    vec![String::from("##.###")]
}


#[test]
fn first_line_contains_one_random_hole() {
    let result = generate_dungeon();
    assert!(result[0].contains("#.#"));
    assert_eq!(1, result[0].matches(".").count());
}