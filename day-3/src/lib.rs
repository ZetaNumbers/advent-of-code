use std::cmp;

const INPUT: &str = include_str!("../input.txt");

pub fn part_1() -> u32 {
    let input = parse_input();

    let mut hist = Histogram::default();
    for &word in &input {
        let word = decompress_word(word);
        for (dst, bit) in hist.iter_mut().zip(word) {
            dst[bit as usize] += 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for stats in hist {
        let [gamma_bit, epsilon_bit] = match stats[0].cmp(&stats[1]) {
            cmp::Ordering::Less => [1, 0],
            cmp::Ordering::Equal => unreachable!(),
            cmp::Ordering::Greater => [0, 1],
        };

        gamma = gamma << 1 | gamma_bit;
        epsilon = epsilon << 1 | epsilon_bit;
    }

    gamma * epsilon
}

type Word = [bool; 12];
type CompactWord = u16;
type Histogram = [[u32; 2]; 12];

fn parse_input() -> Vec<CompactWord> {
    INPUT
        .lines()
        .map(|line| compress_word(&parse_word(line)))
        .collect()
}

fn parse_word(s: &str) -> Word {
    let mut out = Word::default();
    let mut iter = s.bytes().map(|ch| match ch {
        b'0' => false,
        b'1' => true,
        _ => unreachable!(),
    });

    for (dst, bit) in out.iter_mut().zip(&mut iter) {
        *dst = bit;
    }

    assert_eq!(iter.next(), None, "word should only be 12 bits long");
    out
}

fn compress_word(v: &Word) -> CompactWord {
    v.iter().fold(0, |acc, &bit| acc << 1 | bit as u16)
}

fn decompress_word(v: CompactWord) -> Word {
    let mut out = Word::default();
    out.iter_mut().rev().enumerate().for_each(|(i, dst)| {
        *dst = v >> i & 1 == 1;
    });
    out
}

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
        let out = format!("{:012b}", compress_word(&parse_word(s)));
        assert_eq!(s, out);
    }

    #[test]
    fn pt1() {
        assert_eq!(part_1(), 2724524)
    }
}
