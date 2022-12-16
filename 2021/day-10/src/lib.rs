pub fn part_1(input: &str) -> u64 {
    assert!(input.is_ascii());

    let mut score = 0;

    for line in input.lines() {
        let mut stack = Vec::new();
        for ch in line.bytes() {
            let (char, state) = ChunkChar::from_char(ch).unwrap();
            match state {
                ChunkState::Opening => stack.push(char),
                ChunkState::Closing => {
                    if let Some(&last) = stack.last() {
                        if last != char {
                            score += char.points_1();
                            break;
                        }
                        stack.pop();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    score
}

pub fn part_2(input: &str) -> u64 {
    assert!(input.is_ascii());

    let mut scores = Vec::new();

    'lines: for line in input.lines() {
        let mut stack = Vec::new();
        for ch in line.bytes() {
            let (char, state) = ChunkChar::from_char(ch).unwrap();
            match state {
                ChunkState::Opening => stack.push(char),
                ChunkState::Closing => {
                    if let Some(&last) = stack.last() {
                        if last != char {
                            continue 'lines;
                        }
                        stack.pop();
                    } else {
                        continue 'lines;
                    }
                }
            }
        }
        let mut score = 0;
        for &ch in stack.iter().rev() {
            score = score * 5 + ch.points_2();
        }
        scores.push(score);
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChunkChar {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Clone, Copy)]
enum ChunkState {
    Opening,
    Closing,
}

impl ChunkChar {
    fn from_char(ch: u8) -> Option<(ChunkChar, ChunkState)> {
        match ch {
            b'[' => Some((ChunkChar::Square, ChunkState::Opening)),
            b']' => Some((ChunkChar::Square, ChunkState::Closing)),
            b'(' => Some((ChunkChar::Round, ChunkState::Opening)),
            b')' => Some((ChunkChar::Round, ChunkState::Closing)),
            b'{' => Some((ChunkChar::Curly, ChunkState::Opening)),
            b'}' => Some((ChunkChar::Curly, ChunkState::Closing)),
            b'<' => Some((ChunkChar::Angle, ChunkState::Opening)),
            b'>' => Some((ChunkChar::Angle, ChunkState::Closing)),
            _ => None,
        }
    }

    fn points_1(self) -> u64 {
        match self {
            ChunkChar::Round => 3,
            ChunkChar::Square => 57,
            ChunkChar::Curly => 1197,
            ChunkChar::Angle => 25137,
        }
    }

    fn points_2(self) -> u64 {
        match self {
            ChunkChar::Round => 1,
            ChunkChar::Square => 2,
            ChunkChar::Curly => 3,
            ChunkChar::Angle => 4,
        }
    }
}
