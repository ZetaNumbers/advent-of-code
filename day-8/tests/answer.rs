const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_8::part_1(INPUT), 416)
}

#[test]
fn part_2() {
    assert_eq!(day_8::part_2(INPUT), 1043697)
}
