const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_14::common(INPUT, 10), 2112)
}

#[test]
fn part_2() {
    assert_eq!(day_14::common(INPUT, 40), 3243771149914)
}
