const INPUT: &str = include_str!("practice-input.txt");

#[test]
fn part_1() {
    assert_eq!(day_6::solve(INPUT).nth(18).unwrap(), 26);
    assert_eq!(day_6::solve(INPUT).nth(80).unwrap(), 5934);
}

#[test]
fn part_2() {
    assert_eq!(day_6::solve(INPUT).nth(256).unwrap(), 26984457539);
}
