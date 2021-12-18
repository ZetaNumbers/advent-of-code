#[test]
fn part_2() {
    assert_eq!(
        day_17::part_2(day_17::Target {
            x: 20..=30,
            y: -10..=-5
        }),
        112
    )
}
