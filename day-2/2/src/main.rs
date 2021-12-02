use std::str::FromStr;

const INPUT: &str = include_str!("../../input.txt");

enum Command {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Command::Forward),
            "down" => Ok(Command::Down),
            "up" => Ok(Command::Up),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in INPUT.lines() {
        let (cmd, n) = line.split_once(' ').unwrap();

        let n: i32 = n.parse().unwrap();
        match cmd.parse().unwrap() {
            Command::Forward => {
                horiz += n;
                depth += aim * n;
            }
            Command::Down => aim += n,
            Command::Up => aim -= n,
        }
    }

    println!("The answer is: {}", horiz * depth);
}
