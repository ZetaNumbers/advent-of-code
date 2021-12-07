const INPUT: &str = include_str!("practice_input.txt");

#[test]
fn part_1() {
    assert_eq!(day_5::part_1(INPUT.into()), 5)
}

#[test]
fn part_2() {
    assert_eq!(day_5::part_2(INPUT.into()), 12)
}
