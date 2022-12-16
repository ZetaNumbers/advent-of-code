const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_14::common(INPUT, 10), 1588)
}

#[test]
fn part_2() {
    assert_eq!(day_14::common(INPUT, 40), 2188189693529)
}
