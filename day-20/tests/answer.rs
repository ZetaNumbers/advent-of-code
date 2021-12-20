const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_20::common(INPUT, 2), 5057)
}

#[test]
fn part_2() {
    assert_eq!(day_20::common(INPUT, 50), 18502)
}
