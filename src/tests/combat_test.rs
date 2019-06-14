#[test]
fn combat_resolves_1() {
    // given a guard at 11, 12 with:
    // - HP 20
    // - attack 5
    // - defense 10

    // given a player at ?? with:
    // - HP 100
    // - attack 10
    // - defense 15

    // Given player attack roll 15
    // Given damage roll 5
    // Given guard attack roll 16
    // Given damage roll 7

    // Given player attack roll > guard defense
    // Given guard attack roll > player defense
    // Then:
    // Guard takes damage (min 1 max 10)
    // Player takes damage (min 1 max 10)

    // Then:
    // Guard HP is 15
    // Player HP is 93
}