use nalgebra as na;

pub fn part_1(input: &str) -> u64 {
    let input = parse_input(input);

    let mut score = 0;
    for i in 0..input.nrows() {
        for j in 0..input.ncols() {
            let h0 = input[(i, j)];
            println!("# {:?}: {}", (i, j), h0);

            if local((i, j))
                .into_iter()
                .filter_map(|pos| {
                    let d = input.get(pos);
                    println!(" - {:?}: {:?}", pos, d);
                    d
                })
                .all(|&h| h0 < h)
            {
                println!("deep");
                score += h0 as u64 + 1;
            }
        }
    }

    score
}

pub fn part_2(input: &str) -> u64 {
    let input = parse_input(input);
    let mut graph = input.map(|h| (h < 9).then(|| false));
    let mut max_sizes = [1, 1, 1];

    for i in 0..graph.nrows() {
        for j in 0..graph.ncols() {
            let size = mark_basin(&mut graph, (i, j)).unwrap();

            if max_sizes[0] < size {
                max_sizes[0] = size;
                max_sizes.sort();
            }
        }
    }

    max_sizes[0] * max_sizes[1] * max_sizes[2]
}

fn parse_input(input: &str) -> na::DMatrix<u8> {
    assert!(input.is_ascii());
    let columns = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    na::DMatrix::from_iterator(
        columns,
        rows,
        input.lines().flat_map(|line| {
            line.bytes().map(|ch| {
                let digit = ch - b'0';
                assert!(digit < 10);
                digit
            })
        }),
    )
    .transpose()
}

fn mark_basin(graph: &mut na::DMatrix<Option<bool>>, pos: (usize, usize)) -> Option<u64> {
    match graph.get_mut(pos)? {
        Some(true) | None => return Some(0),
        Some(cell @ false) => *cell = true,
    }

    Some(
        local(pos)
            .into_iter()
            .map(|pos| mark_basin(graph, pos).unwrap_or_default())
            .sum::<u64>()
            + 1,
    )
}

fn local((i, j): (usize, usize)) -> [(usize, usize); 4] {
    [
        (i, j.wrapping_sub(1)),
        (i.wrapping_sub(1), j),
        (i + 1, j),
        (i, j + 1),
    ]
}
