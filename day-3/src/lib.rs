use nalgebra::{SMatrix, SVector};

const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_ascii() {
        assert!(INPUT.is_ascii())
    }

    #[test]
    fn parsing_word() {
        let s = "010011010000";
        let out = format!("{:012b}", word_to_u32(&parse_word(s)));
        assert_eq!(s, out);
    }

    #[test]
    fn pt1() {
        assert_eq!(part_1(), 2724524)
    }
}

pub fn part_1() -> u32 {
    let input = parse_input();

    let hist: SMatrix<u32, 12, 2> = input
        .map(|word| SMatrix::from_columns(&[word.map(|x| !x as u32), word.map(|x| x as u32)]))
        .sum();

    let gamma = word_to_u32(&SVector::from_iterator(
        hist.row_iter().map(|h| h[0] < h[1]),
    ));
    let epsilon = word_to_u32(&SVector::from_iterator(
        hist.row_iter().map(|h| h[0] > h[1]),
    ));

    assert_eq!(!gamma & 0xfff, epsilon);

    gamma * epsilon
}

fn parse_input() -> impl Iterator<Item = SVector<bool, 12>> {
    INPUT.lines().map(parse_word)
}

fn parse_word(s: &str) -> SVector<bool, 12> {
    let mut iter = s.bytes().map(|ch| match ch {
        b'0' => false,
        b'1' => true,
        _ => unreachable!(),
    });
    let out = SVector::from_iterator(&mut iter);
    assert_eq!(iter.next(), None, "word should only be 12 bits long");
    out
}

fn word_to_u32(v: &SVector<bool, 12>) -> u32 {
    v.iter().fold(0, |acc, &bit| acc << 1 | bit as u32)
}
