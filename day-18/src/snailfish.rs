use smallbitvec::SmallBitVec;
use std::{
    mem,
    ops::{self, ControlFlow},
};

#[derive(Clone)]
pub enum Number {
    Regular(u8),
    Pair(Box<[Number; 2]>),
}

impl Number {
    fn add_no_reduce(self, other: Self) -> Self {
        Number::Pair(Box::new([self, other]))
    }

    fn reduce(&mut self) {
        loop {
            if let Some(i) = self.first_to_explode() {
                self.explode(i);
            } else if !self.try_split() {
                break;
            }
        }
    }

    fn first_to_explode(&self) -> Option<Index> {
        fn impl_(num: &Number, mut idx: Index) -> ControlFlow<Index> {
            match num {
                Number::Regular(_) => ControlFlow::Continue(()),
                Number::Pair(p) => {
                    if idx.len() >= 4 {
                        ControlFlow::Break(idx)
                    } else {
                        idx.push(false);
                        impl_(&p[0], idx.clone())?;

                        idx.set_last(true);
                        impl_(&p[1], idx)
                    }
                }
            }
        }

        match impl_(self, Index::default()) {
            ControlFlow::Continue(()) => None,
            ControlFlow::Break(i) => Some(i),
        }
    }

    fn explode(&mut self, idx: Index) {
        let pair = self.get_mut(idx.clone()).unwrap();
        let pair = mem::replace(pair, Number::Regular(0));
        let [a, b] = match pair {
            Number::Pair(mut p) => match &mut *p {
                [Number::Regular(a), Number::Regular(b)] => [*a, *b],
                _ => panic!(),
            },
            _ => panic!(),
        };
        if let Some(left) = idx.clone().first_left_shallow() {
            *self.get_mut(left).unwrap().last_mut() += a;
        }
        if let Some(right) = idx.first_right_shallow() {
            *self.get_mut(right).unwrap().first_mut() += b;
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Number::Regular(num) => {
                if *num > 9 {
                    let a = *num / 2;
                    let b = a + *num % 2;
                    *self = Number::Pair(Box::new([Number::Regular(a), Number::Regular(b)]));
                    true
                } else {
                    false
                }
            }
            Number::Pair(p) => p[0].try_split() || p[1].try_split(),
        }
    }

    fn get_mut(&mut self, mut idx: Index) -> Option<&mut Self> {
        if let Some(first) = idx.0.get(0) {
            match self {
                Number::Regular(_) => None,
                Number::Pair(p) => {
                    idx.0.remove(0);
                    p[usize::from(first)].get_mut(idx)
                }
            }
        } else {
            Some(self)
        }
    }

    fn first_mut(&mut self) -> &mut u8 {
        self.edge_mut(false)
    }

    fn last_mut(&mut self) -> &mut u8 {
        self.edge_mut(true)
    }

    fn edge_mut(&mut self, direction: bool) -> &mut u8 {
        let mut this = self;
        loop {
            match this {
                Number::Regular(n) => break n,
                Number::Pair(p) => this = &mut p[usize::from(direction)],
            }
        }
    }

    pub fn from_str(s: &str) -> Self {
        assert!(s.is_ascii());
        let mut s = s.as_bytes();

        fn impl_(s: &mut &[u8]) -> Number {
            let ch;
            (ch, *s) = s.split_first().unwrap();

            if let b'[' = ch {
                let a = impl_(s);

                let delim;
                (delim, *s) = s.split_first().unwrap();
                assert_eq!(*delim, b',');

                let b = impl_(s);

                let closing;
                (closing, *s) = s.split_first().unwrap();
                assert_eq!(*closing, b']');

                Number::Pair(Box::new([a, b]))
            } else {
                let d = ch - b'0';
                assert!(d < 10);
                Number::Regular(d)
            }
        }

        impl_(&mut s)
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            Number::Regular(num) => u64::from(*num),
            Number::Pair(p) => 3 * p[0].magnitude() + 2 * p[1].magnitude(),
        }
    }
}

impl ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.add_no_reduce(rhs);
        out.reduce();
        out
    }
}

#[derive(Clone, Default)]
struct Index(SmallBitVec);

impl Index {
    fn first_left_shallow(self) -> Option<Self> {
        self.first_shallow(false)
    }

    fn first_right_shallow(self) -> Option<Self> {
        self.first_shallow(true)
    }

    fn first_shallow(mut self, direction: bool) -> Option<Self> {
        loop {
            if self.0.last()? != direction {
                self.set_last(direction);
                break Some(self);
            } else {
                self.0.pop();
            }
        }
    }

    fn push(&mut self, last: bool) {
        self.0.push(last);
    }

    fn set_last(&mut self, bit: bool) {
        self.0.set(self.0.len() - 1, bit)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
