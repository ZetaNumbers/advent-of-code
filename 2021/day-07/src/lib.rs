pub fn part_1(input: &str) -> u32 {
    let mut input = parse_input(input);
    let k = input.len() / 2;
    input.sort();
    let median = input[k];

    input.iter().map(|&x| (x - median).abs() as u32).sum()
}

pub fn part_2(input: &str) -> u64 {
    let input = parse_input(input);
    let min = input.iter().copied().min().unwrap();
    let max = input.iter().copied().max().unwrap();
    (min..=max)
        .map(|x0| {
            input
                .iter()
                .map(|&x| {
                    let d = (x - x0).abs() as u64;
                    (d * d + d) / 2
                })
                .sum::<u64>()
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}
