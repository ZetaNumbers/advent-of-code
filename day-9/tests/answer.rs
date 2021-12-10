const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_9::part_1(INPUT), 558)
}

#[test]
fn part_2() {
    assert_eq!(day_9::part_2(INPUT), 882942)
}
