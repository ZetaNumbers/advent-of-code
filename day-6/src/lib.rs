use std::{iter, mem};

pub fn solve(input: &str) -> impl Iterator<Item = u64> {
    let mut histogram = [0_u64; 9];
    for days_left in parse_input(input) {
        histogram[usize::from(days_left)] += 1
    }

    iter::from_fn(move || {
        let mut new_histogram = [0; 9];
        new_histogram[..8].copy_from_slice(&histogram[1..]);
        new_histogram[6] += histogram[0];
        new_histogram[8] += histogram[0];
        Some(
            mem::replace(&mut histogram, new_histogram)
                .into_iter()
                .sum(),
        )
    })
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = u8> + 'a {
    input.split(',').map(|n| n.trim().parse().unwrap())
}
