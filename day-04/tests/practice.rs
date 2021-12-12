use day_04::PRACTICE_INPUT as INPUT;

#[test]
fn part_1() {
    assert_eq!(day_04::part_1(INPUT.parse().unwrap()), 4512);
}

#[test]
fn part_2() {
    assert_eq!(day_04::part_2(INPUT.parse().unwrap()), 1924);
}
