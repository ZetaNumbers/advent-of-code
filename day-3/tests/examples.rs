use day_3::{parse_input, TOY_INPUT};

#[test]
fn part_1() {
    assert_eq!(day_3::part_1(parse_input(TOY_INPUT)), 198)
}

#[test]
fn part_2() {
    assert_eq!(day_3::part_2(parse_input(TOY_INPUT)), 230)
}
