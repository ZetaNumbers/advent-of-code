use std::cell::Cell;

use nalgebra as na;

pub fn common(input: &str, level: u32) -> u64 {
    let (algo, mut image) = parse_input(input);
    let invert = algo[0];
    let algo = if invert {
        let mut even = Box::clone(&algo);
        let mut odd = algo;
        invert_out_algo(&mut even);
        invert_in_algo(&mut odd);
        (even, odd)
    } else {
        (Box::clone(&algo), algo)
    };

    assert_eq!(level % 2, 0);

    for _ in 0..level / 2 {
        image = enchance(&algo.0, &image);
        image = enchance(&algo.1, &image);
    }

    image.fold(0, |acc, b| acc + b as u64)
}

#[allow(dead_code)]
fn debug_image(image: &na::DMatrix<bool>) {
    for row in image.row_iter() {
        for px in row.iter() {
            print!("{}", if *px { '#' } else { '.' })
        }
        println!()
    }
    println!()
}

fn invert_out_algo(algo: &mut [bool; 512]) {
    algo.iter_mut().for_each(|b| *b = !*b);
}

fn invert_in_algo(algo: &mut [bool; 512]) {
    let algo = Cell::from_mut(&mut algo[..]).as_slice_of_cells();
    algo[..256]
        .iter()
        .zip(algo[256..].iter().rev())
        .for_each(|(a, b)| a.swap(b));
}

fn enchance(algo: &[bool; 512], image: &na::DMatrix<bool>) -> na::DMatrix<bool> {
    na::DMatrix::from_fn(image.nrows() + 2, image.ncols() + 2, |i, j| {
        algo[algo_index(image, (i.wrapping_sub(1), j.wrapping_sub(1)))]
    })
}

fn algo_index(image: &na::DMatrix<bool>, pos: (usize, usize)) -> usize {
    local(pos)
        .iter()
        .map(|pos| image.get(*pos).copied().unwrap_or_default())
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc | (b as usize) << i)
}

#[rustfmt::skip]
fn local((i, j): (usize, usize)) -> [(usize, usize); 9]{ [
    (i.wrapping_sub(1), j.wrapping_sub(1)),
    (i.wrapping_sub(1), j                ),
    (i.wrapping_sub(1), j.wrapping_add(1)),
    (i                , j.wrapping_sub(1)),
    (i                , j                ),
    (i                , j.wrapping_add(1)),
    (i.wrapping_add(1), j.wrapping_sub(1)),
    (i.wrapping_add(1), j                ),
    (i.wrapping_add(1), j.wrapping_add(1)),
]}

fn parse_input(input: &str) -> (Box<[bool; 512]>, na::DMatrix<bool>) {
    let mut lines = input.lines();

    let algo: Box<[bool]> = lines.next().unwrap().chars().map(parse_char).collect();

    assert_eq!(lines.next(), Some(""));

    let rows = lines.clone().count();
    let cols = lines.clone().next().unwrap_or("").len();

    let mut image = na::DMatrix::from_iterator(
        rows,
        cols,
        lines.flat_map(|line| line.chars().map(parse_char)),
    );
    image.transpose_mut();

    (algo.try_into().unwrap(), image)
}

fn parse_char(ch: char) -> bool {
    match ch {
        '.' => false,
        '#' => true,
        _ => unreachable!(),
    }
}
