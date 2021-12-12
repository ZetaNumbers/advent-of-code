use day_03::{parse_input, EXAMPLE_INPUT as INPUT};

#[test]
fn part_1() {
    assert_eq!(day_03::part_1(parse_input(INPUT)), 198)
}

#[test]
fn part_2() {
    assert_eq!(day_03::part_2(parse_input(INPUT)), 230)
}
