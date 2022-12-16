use itertools::Itertools;

mod snailfish;

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|s| snailfish::Number::from_str(s))
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|s| snailfish::Number::from_str(s))
        .tuple_combinations()
        .map(|(a, b)| (a.clone() + b.clone()).magnitude().max((b + a).magnitude()))
        .max()
        .unwrap()
}
