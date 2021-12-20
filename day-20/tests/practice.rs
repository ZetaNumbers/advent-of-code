const INPUT: &str = include_str!("example.txt");

#[test]
fn part_1() {
    assert_eq!(day_20::common(INPUT, 2), 35)
}

#[test]
fn part_2() {
    assert_eq!(day_20::common(INPUT, 50), 3351)
}
