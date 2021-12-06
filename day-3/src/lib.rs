use std::cmp;

pub const INPUT: &str = include_str!("../input.txt");
pub const TOY_INPUT: &str = include_str!("../toy-input.txt");

pub fn parse_input(s: &str) -> Vec<u32> {
    s.lines().map(parse_word).collect()
}

pub fn part_1(input: Vec<u32>) -> u32 {
    let mut histogram = [[0_u32; 2]; 32];
    for &word in &input {
        for (i, dst) in histogram.iter_mut().enumerate() {
            let bit = word >> i & 1;
            dst[bit as usize] += 1;
        }
    }

    let (gamma, epsilon) = histogram
        .iter()
        .rev()
        .fold((0, 0), |(gamma, epsilon), stats| {
            dbg!(stats);
            let most_common_bit = most_common_bit(*stats, || panic!()) as u32;
            let least_common_bit = least_common_bit(*stats, || panic!()) as u32;
            dbg!(most_common_bit);
            dbg!(least_common_bit);
            (
                gamma << 1 | most_common_bit,
                epsilon << 1 | least_common_bit,
            )
        });

    gamma * epsilon
}

pub fn part_2(input: Vec<u32>) -> u32 {
    let oxigen_rate = lifesystem_rate(input.clone(), |stats| {
        cmp::max_by_key(false, true, |&b| stats[b as usize])
    });
    let co2_rate = lifesystem_rate(input, |stats| {
        cmp::min_by_key(true, false, |&b| stats[b as usize])
    });

    oxigen_rate * co2_rate
}

fn parse_word(s: &str) -> u32 {
    assert!(s.len() <= 32);

    s.bytes().rev().fold(0, |acc, ch| {
        let bit = match ch {
            b'0' => 0,
            b'1' => 1,
            _ => unreachable!(),
        };
        (acc << 1) | bit
    })
}

fn lifesystem_rate<F>(mut candidates: Vec<u32>, mut criteria: F) -> u32
where
    F: FnMut([u32; 2]) -> bool,
{
    let mut criteria_pos = 12;
    loop {
        match candidates.as_slice() {
            [rate] => break *rate as u32,
            [] => unreachable!(),
            _ => (),
        }
        criteria_pos -= 1;

        let mut stats = [0; 2];
        candidates.iter().for_each(|w| {
            let bit = w >> criteria_pos & 1;
            stats[bit as usize] += 1;
        });

        let criteria_bit = match stats {
            [0, _] => true,
            [_, 0] => false,
            stats => criteria([stats[0], stats[1]]),
        } as u32;

        candidates = candidates
            .iter()
            .copied()
            .filter(|w| (w >> criteria_pos & 1) == criteria_bit)
            .collect();
    }
}

fn least_common_bit<F>(stats: [u32; 2], on_equals: F) -> bool
where
    F: FnOnce() -> bool,
{
    match stats {
        [a, b] if a == b => on_equals(),
        [0, _] => true,
        [_, 0] => false,
        stats => cmp::min_by_key(false, true, |&b| stats[b as usize]),
    }
}

fn most_common_bit<F>(stats: [u32; 2], on_equals: F) -> bool
where
    F: FnOnce() -> bool,
{
    match stats {
        [a, b] if a == b => on_equals(),
        [0, _] => true,
        [_, 0] => false,
        stats => cmp::max_by_key(false, true, |&b| stats[b as usize]),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parsing_word() {
        let mut s = b"010011010000".to_owned();
        let out = format!(
            "{:012b}",
            crate::parse_word(std::str::from_utf8(&s).unwrap())
        );
        s.reverse();
        assert_eq!(s, out.as_bytes());
    }
}
