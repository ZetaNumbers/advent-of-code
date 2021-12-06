use nalgebra as na;

pub const INPUT: &str = include_str!("../input.txt");
pub const EXAMPLE_INPUT: &str = include_str!("../example.txt");

pub fn parse_input(s: &str) -> na::DMatrix<bool> {
    let nrows = match s.lines().next() {
        Some(line) => line.len(),
        None => return na::DMatrix::from_element(0, 0, false),
    };

    let mut data = Vec::with_capacity(nrows);
    for line in s.lines() {
        assert_eq!(line.len(), nrows);
        data.extend(parse_word(line));
    }

    assert_eq!(data.len() % nrows, 0);
    let ncols = data.len() / nrows;

    return na::DMatrix::from_vec(nrows, ncols, data);

    fn parse_word<'a>(s: &'a str) -> impl Iterator<Item = bool> + 'a {
        s.bytes().map(|ch| match ch {
            b'0' => false,
            b'1' => true,
            _ => unreachable!(),
        })
    }
}

pub fn part_1(input: na::DMatrix<bool>) -> u32 {
    let mut histogram = na::MatrixXx2::from_element(input.nrows(), 0_u32);
    let mut delta = na::MatrixXx2::from_element(input.nrows(), 0);
    for word in input.column_iter() {
        let mut word_iter = word.iter();
        delta
            .column_mut(0)
            .as_mut_slice()
            .fill_with(|| !*word_iter.next().unwrap() as u32);
        let mut word_iter = word.iter();
        delta
            .column_mut(1)
            .as_mut_slice()
            .fill_with(|| *word_iter.next().unwrap() as u32);
        histogram += &delta;
    }

    let gamma = word_to_number(
        histogram
            .row_iter()
            .map(|stats| most_common_bit(stats[0], stats[1], |_| panic!())),
    );
    let epsilon = word_to_number(
        histogram
            .row_iter()
            .map(|stats| least_common_bit(stats[0], stats[1], |_| panic!())),
    );

    gamma * epsilon
}

pub fn part_2(input: na::DMatrix<bool>) -> u32 {
    let oxygen_rating = gas_rating(input.clone(), |zeros, ones| {
        most_common_bit(zeros, ones, |_| true)
    });
    let co2_rating = gas_rating(input, |zeros, ones| {
        least_common_bit(zeros, ones, |_| false)
    });

    let oxygen_rating = word_to_number(oxygen_rating.iter().copied());
    let co2_rating = word_to_number(co2_rating.iter().copied());
    return oxygen_rating * co2_rating;

    fn gas_rating<F>(mut input: na::DMatrix<bool>, mut criteria: F) -> na::DVector<bool>
    where
        F: FnMut(u32, u32) -> bool,
    {
        let mut filter = na::DVector::from_element(0, false);

        loop {
            match input.ncols() {
                0 => unreachable!(),
                1 => break input.column(0).into(),
                _ => (),
            }

            let stats = input
                .row(filter.nrows())
                .fold(na::vector![0, 0], |acc, bit| {
                    acc + if bit {
                        na::vector![0, 1]
                    } else {
                        na::vector![1, 0]
                    }
                });

            filter = filter.push(criteria(stats[0], stats[1]));

            let mut data = Vec::with_capacity(input.nrows() * input.ncols());
            for w in input.column_iter() {
                if !w.as_slice().starts_with(filter.as_slice()) {
                    continue;
                }
                data.extend_from_slice(w.as_slice());
            }

            assert_eq!(data.len() % input.nrows(), 0);
            let ncols = data.len() / input.nrows();
            input = na::DMatrix::from_vec(input.nrows(), ncols, data);
        }
    }
}

fn word_to_number(word: impl Iterator<Item = bool>) -> u32 {
    word.fold(0, |acc, bit| acc << 1 | bit as u32)
}

fn most_common_bit<F>(zeros: u32, ones: u32, on_equals: F) -> bool
where
    F: FnOnce(u32) -> bool,
{
    match [zeros, ones] {
        [a, b] if a == b => on_equals(a),
        stats => std::cmp::max_by_key(false, true, |&i| stats[i as usize]),
    }
}

fn least_common_bit<F>(zeros: u32, ones: u32, on_equals: F) -> bool
where
    F: FnOnce(u32) -> bool,
{
    match [zeros, ones] {
        [a, b] if a == b => on_equals(a),
        [0, _] | [_, 0] => unimplemented!(),
        stats => std::cmp::min_by_key(false, true, |&i| stats[i as usize]),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn input_is_ascii() {
        assert!(super::INPUT.is_ascii())
    }
}
