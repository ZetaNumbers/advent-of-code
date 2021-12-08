const INPUT: &str = include_str!("practice-input.txt");

#[test]
fn part_1() {
    assert_eq!(day_7::part_1(INPUT), 37)
}

#[test]
fn part_2() {
    assert_eq!(day_7::part_2(INPUT), 168)
}
