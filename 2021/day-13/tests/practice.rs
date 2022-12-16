const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_13::part_1(INPUT), 17)
}
