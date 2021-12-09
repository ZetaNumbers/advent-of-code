use crate::display7::Display7;

mod display7;

pub fn part_1(input: &str) -> u64 {
    let input = parse_input(input);
    input
        .map(|line| {
            line.1
                .iter()
                .filter(|o| matches!(o.len(), 2 | 3 | 4 | 7))
                .count() as u64
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut i = 0;

    parse_input(input)
        .map(|(patterns, output)| {
            let patterns = patterns.map(Display7::parse);
            let output = output.map(Display7::parse);

            let [one] = filter_patterns_with_count(&patterns, 2);
            let [seven] = filter_patterns_with_count(&patterns, 3);
            let [four] = filter_patterns_with_count(&patterns, 4);
            let two_three_five = filter_patterns_with_count::<3>(&patterns, 5);
            let zero_six_nine = filter_patterns_with_count::<3>(&patterns, 6);
            let [eight] = filter_patterns_with_count(&patterns, 7);

            let horiz = two_three_five[0] & two_three_five[1] & two_three_five[2];
            let vert = !horiz;
            let left_vert = vert ^ one;

            let three = horiz | one;
            let five = (zero_six_nine[0] & zero_six_nine[1] & zero_six_nine[2]) | horiz;
            let six = five | left_vert;
            let nine = four | horiz;
            let two = !five | horiz;
            let zero = eight ^ (four & horiz);

            let digits = [zero, one, two, three, four, five, six, seven, eight, nine];

            println!("#{}", i);
            i += 1;

            assert!(two_three_five.contains(&three));
            assert!(two_three_five.contains(&five));
            assert!(two_three_five.contains(&two));

            output
                .into_iter()
                .map(|o| o.decypher(&digits))
                .fold(0, |acc, d| acc * 10 + d as u64)
        })
        .sum()
}

fn filter_patterns_with_count<const N: usize>(
    patterns: &[Display7; 10],
    count: u32,
) -> [Display7; N] {
    let iter = patterns.iter().copied().filter(|&d| d.count() == count);
    array_from_iter_exact(iter)
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = ([&'a str; 10], [&'a str; 4])> + 'a {
    input.lines().map(|line| {
        let (p, o) = line.split_once(" | ").unwrap();
        let mut p_iter = p.split_ascii_whitespace();
        let mut o_iter = o.split_ascii_whitespace();

        let patterns = [(); 10].map(|()| p_iter.next().unwrap());
        let output = [(); 4].map(|()| o_iter.next().unwrap());

        assert_eq!(p_iter.next(), None);
        assert_eq!(o_iter.next(), None);

        (patterns, output)
    })
}

fn array_from_iter_exact<I, T, const N: usize>(iter: I) -> [T; N]
where
    I: IntoIterator<Item = T>,
{
    let mut iter = iter.into_iter();
    let out = [(); N].map(|()| iter.next().unwrap());
    assert!(iter.next().is_none());
    out
}
