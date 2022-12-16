const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_05::part_1(INPUT.into()), 6113)
}

#[test]
fn part_2() {
    assert_eq!(day_05::part_2(INPUT.into()), 20373)
}
