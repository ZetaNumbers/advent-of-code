use std::collections::BTreeSet;

use itertools::Itertools;

pub fn most_calories_per_elf(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(empty, g)| (!empty).then_some(g))
        .map(|elf| elf.map(|l| l.parse::<u64>().unwrap()).sum())
        .max()
}

pub fn most_calories_top_3(input: &str) -> Option<u64> {
    let mut calories: BTreeSet<u64> = input
        .lines()
        .map(|l| l.trim())
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(empty, g)| (!empty).then_some(g))
        .map(|elf| elf.map(|l| l.parse::<u64>().unwrap()).sum())
        .collect();

    Some(calories.pop_last()? + calories.pop_last()? + calories.pop_last()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRACTICE_INPUT: &str = include_str!("practice.txt");
    const ANSWER_INPUT: &str = include_str!("answer.txt");

    #[test]
    fn practice_pt1() {
        assert_eq!(most_calories_per_elf(PRACTICE_INPUT), Some(24000))
    }

    #[test]
    fn answer_pt1() {
        assert_eq!(most_calories_per_elf(ANSWER_INPUT), Some(71471))
    }

    #[test]
    fn practice_pt2() {
        assert_eq!(most_calories_top_3(PRACTICE_INPUT), Some(45000))
    }

    #[test]
    fn answer_pt2() {
        assert_eq!(most_calories_top_3(ANSWER_INPUT), Some(211189))
    }
}
