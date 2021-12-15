use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let (mut points, folds) = parse_input(input);

    let fold = *folds.first().unwrap();
    fold_points(&mut points, fold);

    points.len()
}

pub fn part_2(input: &str) {
    let (mut points, folds) = parse_input(input);

    folds
        .into_iter()
        .for_each(|fold| fold_points(&mut points, fold));

    let max_x = points.iter().map(|&(x, _y)| x).max().unwrap();
    let max_y = points.iter().map(|&(_x, y)| y).max().unwrap();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            print!("{}", if points.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }
}

fn parse_input(input: &str) -> (HashSet<(i16, i16)>, Vec<(FoldAxis, i16)>) {
    let mut lines = input.lines();

    let points = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = lines
        .map(|line| {
            const PREFIX: &str = "fold along ";
            assert!(line.starts_with(PREFIX), "{}", line);
            let (axis, value) = line[PREFIX.len()..].split_once('=').unwrap();
            (axis.parse().unwrap(), value.parse().unwrap())
        })
        .collect();

    (points, folds)
}

fn fold_points(points: &mut HashSet<(i16, i16)>, fold: (FoldAxis, i16)) {
    *points = points
        .iter()
        .map(|&(x, y)| match fold.0 {
            FoldAxis::X => (if x < fold.1 { x } else { fold.1 - (x - fold.1) }, y),
            FoldAxis::Y => (x, if y < fold.1 { y } else { fold.1 - (y - fold.1) }),
        })
        .collect();
}

#[derive(Clone, Copy)]
enum FoldAxis {
    X,
    Y,
}

impl std::str::FromStr for FoldAxis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(FoldAxis::X),
            "y" => Ok(FoldAxis::Y),
            _ => Err(()),
        }
    }
}
