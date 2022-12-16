use itertools::Itertools;
use std::{fmt, ops};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Display7(u8);

impl fmt::Debug for Display7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

impl Display7 {
    pub fn parse(input: &str) -> Self {
        assert!(input.is_ascii());
        let mut out = 0;

        for ch in input.bytes() {
            let code = ch - b'a';
            assert!(code < 7);
            assert!(out >> code & 1 == 0);
            out |= 1 << code;
        }

        Display7(out)
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn decypher(self, abc: &[Display7; 10]) -> u8 {
        abc.iter()
            .enumerate()
            .filter(|(_, &d)| d == self)
            .map(|(i, _)| i as u8)
            .exactly_one()
            .unwrap_or_else(|_| panic!("{:?}: {:?}", self, abc))
    }
}

impl ops::BitAnd for Display7 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Display7(self.0 & rhs.0)
    }
}

impl ops::BitXor for Display7 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Display7(self.0 ^ rhs.0)
    }
}

impl ops::BitOr for Display7 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Display7(self.0 | rhs.0)
    }
}

impl ops::Not for Display7 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Display7(!self.0 & 0x7f)
    }
}
