const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_9::part_1(INPUT), 15)
}

#[test]
fn part_2() {
    assert_eq!(day_9::part_2(INPUT), 1134)
}
