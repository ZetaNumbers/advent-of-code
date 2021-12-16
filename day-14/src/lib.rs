use itertools::Itertools;
use std::{collections::HashMap, mem};

pub fn common(input: &str, steps: u32) -> usize {
    let (template, rules) = parse_input(input);
    let mut hst_pairs = template.as_bytes().windows(2).counts_by(|pair| {
        let &pair: &[u8; 2] = pair.try_into().unwrap();
        pair
    });
    let mut hst = template.as_bytes().iter().copied().counts();
    let mut buffer_pairs = HashMap::new();

    for _ in 0..steps {
        buffer_pairs.values_mut().for_each(|v| *v = 0);

        hst_pairs.iter().for_each(|(k, &v)| {
            if let Some(&i) = rules.get(k) {
                *hst.entry(i).or_default() += v;
                *buffer_pairs.entry([k[0], i]).or_default() += v;
                *buffer_pairs.entry([i, k[1]]).or_default() += v;
            } else {
                *buffer_pairs.entry(*k).or_default() += v;
            }
        });

        mem::swap(&mut hst_pairs, &mut buffer_pairs);
    }

    let (least_common, most_common) = hst.values().minmax().into_option().unwrap();
    most_common - least_common
}

pub fn parse_input(input: &str) -> (String, HashMap<[u8; 2], u8>) {
    assert!(input.is_ascii());
    let mut lines = input.lines();

    let template = lines.next().unwrap().to_owned();
    assert_eq!(lines.next().unwrap(), "");
    let rules = lines
        .map(|line| {
            let (pair, insertion) = line.split_once(" -> ").unwrap();
            let &pair: &[u8; 2] = pair.as_bytes().try_into().unwrap();
            let &[insertion]: &[u8; 1] = insertion.as_bytes().try_into().unwrap();

            (pair, insertion)
        })
        .collect();

    (template, rules)
}
