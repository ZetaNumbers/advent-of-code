const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_6::solve(INPUT).nth(80).unwrap(), 388419)
}

#[test]
fn part_2() {
    assert_eq!(day_6::solve(INPUT).nth(256).unwrap(), 1740449478328)
}
