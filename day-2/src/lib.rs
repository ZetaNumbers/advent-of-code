const INPUT: &str = include_str!("../input.txt");

pub fn part_1() -> i32 {
    let [ans_h, ans_d] = parse_input()
        .into_iter()
        .map(|(cmd, n)| match cmd {
            Command::Forward => [n, 0],
            Command::Down => [0, n],
            Command::Up => [0, -n],
        })
        .reduce(|[h1, d1], [h2, d2]| [h1 + h2, d1 + d2])
        .unwrap();

    ans_h * ans_d
}

pub fn part_2() -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, n) in parse_input() {
        match cmd {
            Command::Forward => {
                horiz += n;
                depth += aim * n;
            }
            Command::Down => aim += n,
            Command::Up => aim -= n,
        }
    }

    horiz * depth
}

fn parse_input() -> Vec<(Command, i32)> {
    INPUT
        .lines()
        .map(|line| {
            let (cmd, n) = line.split_once(' ').unwrap();
            (cmd.parse().unwrap(), n.parse().unwrap())
        })
        .collect()
}

enum Command {
    Forward,
    Down,
    Up,
}

impl std::str::FromStr for Command {
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

#[cfg(test)]
mod tests {
    #[test]
    fn pt1() {
        assert_eq!(super::part_1(), 1507611);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::part_2(), 1880593125);
    }
}
