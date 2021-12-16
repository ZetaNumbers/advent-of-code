const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_15::part_1(INPUT), 508)
}

#[test]
fn part_2() {
    assert_eq!(day_15::part_2(INPUT), 2872)
}
