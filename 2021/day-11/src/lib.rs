use std::iter;

use nalgebra as na;

pub fn part_1(input: &str, iterations: usize) -> u64 {
    let mut octopuses = parse_input(input);
    flash_iterator(&mut octopuses).take(iterations).sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut octopuses = parse_input(input);
    let octopuses_count = octopuses.len() as u64;
    flash_iterator(&mut octopuses)
        .take_while(|&flashes| flashes < octopuses_count)
        .count() as u64
        + 1
}

fn flash_iterator(octopuses: &mut na::DMatrix<u8>) -> impl Iterator<Item = u64> + '_ {
    let mut spread_queue = Vec::new();

    iter::from_fn(move || {
        let mut flashes = 0;

        for j in 0..octopuses.ncols() {
            for i in 0..octopuses.nrows() {
                let octopus = &mut octopuses[(i, j)];
                *octopus += 1;
                if *octopus > 9 {
                    *octopus = 0;
                    flashes += 1;
                    spread_queue.push((i, j));
                }
            }
        }

        while let Some(focus) = spread_queue.pop() {
            for pos in local(focus) {
                let octopus = match octopuses.get_mut(pos) {
                    None | Some(&mut 0) => continue,
                    Some(o) => o,
                };

                *octopus += 1;
                if *octopus > 9 {
                    *octopus = 0;
                    flashes += 1;
                    spread_queue.push(pos);
                }
            }
        }

        Some(flashes)
    })
}

#[rustfmt::skip]
fn local((i, j): (usize, usize)) -> [(usize, usize); 8] {
    [
        (i.wrapping_sub(1), j.wrapping_sub(1)),
        (i    , j.wrapping_sub(1)),
        (i + 1, j.wrapping_sub(1)),
        (i.wrapping_sub(1), j    ),
        (i + 1, j    ),
        (i.wrapping_sub(1), j + 1),
        (i    , j + 1),
        (i + 1, j + 1),
    ]
}

fn parse_input(input: &str) -> na::DMatrix<u8> {
    assert!(input.is_ascii());

    let cols = input.lines().next().unwrap_or_default().len();
    let rows = input.lines().count();

    na::DMatrix::from_iterator(
        rows,
        cols,
        input.lines().flat_map(|line| {
            assert_eq!(line.len(), cols);
            line.bytes().map(|ch| {
                let code = ch - b'0';
                assert!(code < 10);
                code
            })
        }),
    )
    .transpose()
}
