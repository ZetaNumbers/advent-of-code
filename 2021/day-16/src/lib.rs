use std::iter;

use bitvec::prelude::*;

pub fn part_1(input: &str) -> u64 {
    let main = Packet::from_str(input);
    main.version_sum()
}

pub fn part_2(input: &str) -> u64 {
    let main = Packet::from_str(input);
    main.eval()
}

#[derive(Debug)]
struct Packet {
    version: u8,
    body: PacketBody,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        u64::from(self.version)
            + self
                .body
                .as_ref()
                .iter()
                .map(|p| p.version_sum())
                .sum::<u64>()
    }

    fn from_str(s: &str) -> Self {
        let bits = BitVec::<Msb0, u8>::from_iter(s.trim().chars().flat_map(|ch| {
            let digit = ch.to_digit(16).unwrap();
            (0..4).rev().map(move |i| digit >> i & 1 == 1)
        }));
        let mut bits = bits.as_bitslice();

        Self::from_bits(&mut bits)
    }

    fn from_bits(bits: &mut &BitSlice<Msb0, u8>) -> Packet {
        let version;
        (version, *bits) = bits.split_at(3);

        Packet {
            version: version.load_be(),
            body: PacketBody::from_bits(bits),
        }
    }

    fn subpackets_from_bits(bits: &mut &BitSlice<Msb0, u8>) -> Vec<Self> {
        let length_type_id;
        (length_type_id, *bits) = bits.split_at(1);
        match length_type_id[0] {
            false => {
                let sub_bitlen;
                (sub_bitlen, *bits) = bits.split_at(15);
                let sub_bitlen: usize = sub_bitlen.load_be();
                let mut sub_bits;
                (sub_bits, *bits) = bits.split_at(sub_bitlen);
                iter::from_fn(|| (sub_bits.len() > 0).then(|| Self::from_bits(&mut sub_bits)))
                    .collect()
            }
            true => {
                let sub_len;
                (sub_len, *bits) = bits.split_at(11);
                let sub_len: usize = sub_len.load_be();
                (0..sub_len).map(|_| Self::from_bits(bits)).collect()
            }
        }
    }

    fn eval(&self) -> u64 {
        match &self.body {
            PacketBody::Sum(v) => v.iter().map(|p| p.eval()).sum(),
            PacketBody::Product(v) => v.iter().map(|p| p.eval()).product(),
            PacketBody::Min(v) => v.iter().map(|p| p.eval()).min().unwrap(),
            PacketBody::Max(v) => v.iter().map(|p| p.eval()).max().unwrap(),
            PacketBody::Const(c) => *c,
            PacketBody::Greater(p) => (p[0].eval() > p[1].eval()) as u64,
            PacketBody::Less(p) => (p[0].eval() < p[1].eval()) as u64,
            PacketBody::Eq(p) => (p[0].eval() == p[1].eval()) as u64,
        }
    }
}

#[derive(Debug)]
enum PacketBody {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Const(u64),
    Greater(Box<[Packet; 2]>),
    Less(Box<[Packet; 2]>),
    Eq(Box<[Packet; 2]>),
}

impl PacketBody {
    fn from_bits(bits: &mut &BitSlice<Msb0, u8>) -> Self {
        let type_id;
        (type_id, *bits) = bits.split_at(3);

        match type_id.load_be::<u8>() {
            4 => PacketBody::Const({
                let mut num = 0;
                loop {
                    let group;
                    (group, *bits) = bits.split_at(5);
                    num = num << 4 | group[1..].load_be::<u64>();

                    if !group[0] {
                        break num;
                    }
                }
            }),
            type_id @ 0..=7 => {
                let arguments = Packet::subpackets_from_bits(bits);

                match type_id {
                    0 => return PacketBody::Sum(arguments),
                    1 => return PacketBody::Product(arguments),
                    2 => return PacketBody::Min(arguments),
                    3 => return PacketBody::Max(arguments),
                    _ => (),
                };

                let pair = arguments.into_boxed_slice().try_into().unwrap();
                match type_id {
                    5 => PacketBody::Greater(pair),
                    6 => PacketBody::Less(pair),
                    7 => PacketBody::Eq(pair),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl AsRef<[Packet]> for PacketBody {
    fn as_ref(&self) -> &[Packet] {
        match self {
            PacketBody::Sum(v)
            | PacketBody::Product(v)
            | PacketBody::Min(v)
            | PacketBody::Max(v) => v.as_ref(),
            PacketBody::Const(_) => &[],
            PacketBody::Greater(p) | PacketBody::Less(p) | PacketBody::Eq(p) => p.as_ref(),
        }
    }
}
