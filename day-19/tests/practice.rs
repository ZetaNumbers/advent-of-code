const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_19::part_1(INPUT), 79)
}

#[test]
fn part_2() {
    assert_eq!(day_19::part_2(INPUT), 3621.0)
}
