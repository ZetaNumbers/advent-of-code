#[test]
fn part_2() {
    assert_eq!(
        day_17::part_2(day_17::Target {
            x: 169..=206,
            y: -108..=-68
        }),
        2576
    )
}
