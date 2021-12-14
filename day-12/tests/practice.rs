const EXAMPLE_0: &str = include_str!("example-0.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");

#[test]
fn part_1() {
    assert_eq!(day_12::part_1(EXAMPLE_0), 10);
    assert_eq!(day_12::part_1(EXAMPLE_1), 19);
    assert_eq!(day_12::part_1(EXAMPLE_2), 226);
}

#[test]
fn part_2() {
    assert_eq!(day_12::part_2(EXAMPLE_0), 36);
    assert_eq!(day_12::part_2(EXAMPLE_1), 103);
    assert_eq!(day_12::part_2(EXAMPLE_2), 3509);
}
