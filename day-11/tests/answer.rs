const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_11::part_1(INPUT, 100), 1747)
}

#[test]
fn part_2() {
    assert_eq!(day_11::part_2(INPUT), 505)
}
