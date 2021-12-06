use nalgebra as na;

pub const INPUT: &str = include_str!("../input.txt");
pub const PRACTICE_INPUT: &str = include_str!("../practice.txt");

pub fn part_1(input: Input) -> u32 {
    let Input {
        numbers,
        mut boards,
    } = input;

    let mut numbers = numbers.iter();
    loop {
        let number = match numbers.next() {
            Some(&n) => n,
            None => unreachable!(),
        };

        for board in boards.iter_mut() {
            mark(board, number);
            if check_win(board) {
                return calc_score(board, number);
            }
        }
    }
}

pub fn part_2(input: Input) -> u32 {
    let Input {
        numbers,
        mut boards,
    } = input;

    let mut numbers = numbers.iter();
    let mut last_win_board = None;
    loop {
        let number = match numbers.next() {
            Some(&n) => n,
            None => unreachable!(),
        };

        for board in boards.iter_mut() {
            mark(board, number);
        }

        boards.retain(|board| {
            let win = check_win(board);
            if win {
                last_win_board = Some(*board);
            }
            !win
        });

        if boards.is_empty() {
            break calc_score(&last_win_board.unwrap(), number);
        }
    }
}

pub struct Input {
    numbers: Vec<u8>,
    boards: Vec<na::Matrix5<(bool, u8)>>,
}

impl std::str::FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut boards = Vec::new();
        loop {
            match lines.next() {
                None => break,
                Some("") => (),
                Some(_) => unreachable!(),
            }

            boards.push(na::Matrix5::from_iterator((&mut lines).take(5).flat_map(
                |line| {
                    line.split_ascii_whitespace()
                        .map(|num| (false, num.trim().parse().unwrap()))
                },
            )));
        }

        Ok(Input { numbers, boards })
    }
}

fn mark(board: &mut na::Matrix5<(bool, u8)>, number: u8) {
    *board = board.map(|(mark, num)| (mark || num == number, num))
}

fn check_win(board: &na::Matrix5<(bool, u8)>) -> bool {
    for col in board.column_iter() {
        if col.iter().all(|(mark, _)| *mark) {
            return true;
        }
    }

    for row in board.row_iter() {
        if row.iter().all(|(mark, _)| *mark) {
            return true;
        }
    }

    false
}

fn calc_score(board: &na::Matrix5<(bool, u8)>, last_number: u8) -> u32 {
    board
        .iter()
        .filter_map(|(m, n)| (!m).then(|| n))
        .map(|n| *n as u32)
        .sum::<u32>()
        * last_number as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn ascii_input() {
        assert!(super::INPUT.is_ascii());
        assert!(super::PRACTICE_INPUT.is_ascii());
    }
}
