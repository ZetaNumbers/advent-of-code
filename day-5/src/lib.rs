use nalgebra as na;
use std::collections::HashMap;

pub struct Input {
    lines: Vec<na::Matrix2<i16>>,
}

impl From<&'_ str> for Input {
    fn from(s: &str) -> Self {
        Input {
            lines: s
                .lines()
                .map(|line| {
                    na::Matrix2::from_iterator(
                        line.splitn(2, " -> ")
                            .flat_map(|p| p.splitn(2, ','))
                            .map(|c| c.parse().unwrap()),
                    )
                })
                .collect(),
        }
    }
}

pub fn part_1(input: Input) -> u32 {
    let mut diagram: HashMap<na::Vector2<i16>, u32> = HashMap::new();
    input
        .lines
        .into_iter()
        .filter_map(|line| {
            let out = Line::from_4_directions(line);
            if let Some(out) = out {
                println!(
                    "{:?} -> {:?}",
                    line.column(0).as_slice(),
                    line.column(1).as_slice()
                );
                println!(
                    "{:?} -> {} * {:?}",
                    out.origin.as_slice(),
                    out.length,
                    out.step.as_slice()
                );
            }
            out
        })
        .flatten()
        .for_each(|point| {
            println!(" - {:?}", point.as_slice());
            *diagram.entry(point).or_default() += 1
        });

    diagram.values().filter(|&&n| n >= 2).count() as u32
}

pub fn part_2(input: Input) -> u32 {
    let mut diagram: HashMap<na::Vector2<i16>, u32> = HashMap::new();
    input
        .lines
        .into_iter()
        .filter_map(|line| {
            let out = Line::from_8_directions(line);
            if let Some(out) = out {
                println!(
                    "{:?} -> {:?}",
                    line.column(0).as_slice(),
                    line.column(1).as_slice()
                );
                println!(
                    "{:?} -> {} * {:?}",
                    out.origin.as_slice(),
                    out.length,
                    out.step.as_slice()
                );
            }
            out
        })
        .flatten()
        .for_each(|point| {
            println!(" - {:?}", point.as_slice());
            *diagram.entry(point).or_default() += 1
        });

    diagram.values().filter(|&&n| n >= 2).count() as u32
}

#[derive(Clone, Copy)]
struct Line {
    origin: na::Vector2<i16>,
    length: u16,
    step: na::Vector2<i16>,
}

impl Line {
    fn from_8_directions(line: na::Matrix2<i16>) -> Option<Line> {
        let delta = line.column(1) - line.column(0);

        match delta.as_slice() {
            [dx, dy] if dx.abs() == dy.abs() => Some(Line {
                origin: line.column(0).into(),
                length: dx.abs() as u16 + 1,
                step: delta / dx.abs(),
            }),
            _ => Line::from_4_directions(line),
        }
    }

    fn from_4_directions(line: na::Matrix2<i16>) -> Option<Line> {
        let delta = line.column(1) - line.column(0);

        match delta.as_slice() {
            [d, 0] | [0, d] => Some(Line {
                origin: line.column(0).into(),
                length: d.abs() as u16 + 1,
                step: delta / d.abs(),
            }),
            _ => None,
        }
    }
}

// impl TryFrom<na::Matrix2<i16>> for Line {
//     type Error = ();

//     fn try_from(line: na::Matrix2<i16>) -> Result<Self, Self::Error> {

//         match line.as_slice() {
//             &[x, y1, x2, y2] if x == x2 => Ok(CardinalLine {
//                 origin: na::vector![x, y1],
//                 length: (y2 - y1).abs() as u16 + 1,
//                 step: if y1 <= y2 {
//                     na::vector![0, 1]
//                 } else {
//                     na::vector![0, -1]
//                 },
//             }),
//             &[x1, y, x2, y2] if y == y2 => Ok(CardinalLine {
//                 origin: na::vector![x1, y],
//                 length: (x2 - x1).abs() as u16 + 1,
//                 step: if x1 <= x2 {
//                     na::vector![1, 0]
//                 } else {
//                     na::vector![-1, 0]
//                 },
//             }),
//             _ => Err(()),
//         }
//     }
// }

impl Iterator for Line {
    type Item = na::Vector2<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }

        let out = self.origin;
        self.length -= 1;
        self.origin += self.step;
        Some(out)
    }
}
