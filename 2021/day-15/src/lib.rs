use nalgebra as na;
use std::{cell::Cell, cmp, collections::BinaryHeap, ptr};

pub fn part_1(input: &str) -> usize {
    let cavern = parse_input(input);
    find_path(&cavern)
}

pub fn part_2(input: &str) -> usize {
    let cavern = parse_input(input);
    let nrows = cavern.nrows();
    let ncols = cavern.ncols();

    let mut big_cavern = na::DMatrix::repeat(nrows * 5, ncols * 5, 0);
    for i in 0..5 {
        for j in 0..5 {
            big_cavern
                .slice_range_mut(nrows * i..nrows * (i + 1), ncols * j..ncols * (j + 1))
                .copy_from(&cavern.map(|x| (x + i as u8 + j as u8 - 1) % 9 + 1))
        }
    }

    find_path(&big_cavern)
}

fn find_path(parse_input: &na::DMatrix<u8>) -> usize {
    let cavern = parse_input.map_with_location(|i, j, weight| {
        Cell::new(Vertex {
            weight,
            pos: (i, j),
            dist: usize::MAX,
            visit: VisitState::NotVisited,
        })
    });
    let dst = &cavern[(cavern.nrows() - 1, cavern.ncols() - 1)];
    let mut visiting = BinaryHeap::from([cmp::Reverse({
        let src = &cavern[(0, 0)];
        src.replace(Vertex {
            visit: VisitState::Visiting,
            dist: 0,
            ..src.get()
        });
        src
    })]);
    loop {
        let cmp::Reverse(v) = visiting.pop().unwrap();
        v.replace(Vertex {
            visit: VisitState::Visited,
            ..v.get()
        });

        if ptr::eq(v, dst) {
            break v.get().dist;
        }

        local4(v.get().pos)
            .into_iter()
            .filter_map(|pos| {
                cavern
                    .get(pos)
                    .filter(|v| v.get().visit != VisitState::Visited)
            })
            .for_each(|w| {
                if w.get().visit == VisitState::NotVisited {
                    w.replace(Vertex {
                        visit: VisitState::Visiting,
                        ..w.get()
                    });
                    visiting.push(cmp::Reverse(w));
                }

                let dist = v.get().dist + usize::from(w.get().weight);
                if dist < w.get().dist {
                    w.replace(Vertex { dist, ..w.get() });
                }
            });
    }
}

fn parse_input(input: &str) -> na::DMatrix<u8> {
    let size = input.lines().next().map_or(0, str::len);
    let mut out = na::DMatrix::from_iterator(
        size,
        size,
        input
            .lines()
            .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8)),
    );
    out.transpose_mut();
    out
}

fn local4(pos: (usize, usize)) -> [(usize, usize); 4] {
    [
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    dist: usize,
    visit: VisitState,
    pos: (usize, usize),
    weight: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VisitState {
    Visited,
    Visiting,
    NotVisited,
}
