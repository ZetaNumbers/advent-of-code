const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_07::part_1(INPUT), 349812)
}

#[test]
fn part_2() {
    assert_eq!(day_07::part_2(INPUT), 99763899)
}
