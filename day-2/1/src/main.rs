use std::str::FromStr;

const INPUT: &str = include_str!("../../input.txt");

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

fn main() {
    let [ans_h, ans_d] = INPUT
        .lines()
        .map(|p| -> [i32; 2] {
            let (d, n) = p.split_once(' ').unwrap();
            let d = d.parse().unwrap();
            let n = n.parse().unwrap();
            match d {
                Direction::Forward => [n, 0],
                Direction::Down => [0, n],
                Direction::Up => [0, -n],
            }
        })
        .reduce(|[h1, d1], [h2, d2]| [h1 + h2, d1 + d2])
        .unwrap();
    println!("The answer is: {}", ans_h * ans_d);
}
