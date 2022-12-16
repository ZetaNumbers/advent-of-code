const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    assert_eq!(day_10::part_1(INPUT), 278475)
}

#[test]
fn part_2() {
    assert_eq!(day_10::part_2(INPUT), 3015539998)
}
