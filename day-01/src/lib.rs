const INPUT: &'static str = include_str!("../input.txt");

pub fn part_1() -> usize {
    count_increases(&parse_input())
}

pub fn part_2() -> usize {
    let local_sums = parse_input().windows(3).map(|w| w.iter().sum()).collect();
    count_increases(&local_sums)
}

fn parse_input() -> Vec<u32> {
    INPUT
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn count_increases(v: &Vec<u32>) -> usize {
    v.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn pt1() {
        assert_eq!(super::part_1(), 1521)
    }

    #[test]
    fn pt2() {
        assert_eq!(super::part_2(), 1543)
    }
}
