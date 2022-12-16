const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_12::part_1(INPUT), 4720);
}

#[test]
fn part_2() {
    assert_eq!(day_12::part_2(INPUT), 147848);
}
