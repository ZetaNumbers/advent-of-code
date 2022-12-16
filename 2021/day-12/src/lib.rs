use std::{
    collections::{hash_set, HashMap, HashSet},
    iter, mem,
};

pub fn part_1(input: &str) -> u64 {
    let cave_system = cave_system_from_iter(parse_input(input));

    let mut traverse = Traverse::new(&cave_system, false);

    let mut count = 0;
    while traverse.next() {
        count += 1;
    }
    count
}

pub fn part_2(input: &str) -> u64 {
    let cave_system = cave_system_from_iter(parse_input(input));

    let mut traverse = Traverse::new(&cave_system, true);

    let mut count = 0;
    while traverse.next() {
        count += 1;
    }
    count
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str)> {
    assert!(input.is_ascii());
    input.lines().map(|line| line.split_once('-').unwrap())
}

type CaveSystem<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn cave_system_from_iter<'a, I>(iter: I) -> CaveSystem<'a>
where
    I: Iterator<Item = (&'a str, &'a str)> + 'a,
{
    let mut cave_system: CaveSystem = [("start", HashSet::new()), ("end", HashSet::new())].into();

    iter.for_each(|(a, b)| {
        if b != "start" {
            assert!(cave_system.entry(a).or_insert_with(HashSet::new).insert(b))
        }
        if a != "start" {
            assert!(cave_system.entry(b).or_insert_with(HashSet::new).insert(a))
        }
    });

    *cave_system.get_mut("end").unwrap() = HashSet::new();

    cave_system
}

fn is_small_cave(cave: &str) -> bool {
    if cave.chars().all(char::is_lowercase) {
        true
    } else if cave.chars().all(char::is_uppercase) {
        false
    } else {
        unreachable!()
    }
}

struct Traverse<'a> {
    cave_system: &'a CaveSystem<'a>,
    path: Vec<iter::Peekable<hash_set::Iter<'a, &'a str>>>,
    /// Max number of passes per small cave
    extra_pass: Option<&'a str>,
    /// Number of passes per small cave
    passes: HashMap<&'a str, bool>,
}

impl<'a> Traverse<'a> {
    fn new(cave_system: &'a CaveSystem<'a>, extra_pass: bool) -> Self {
        Traverse {
            cave_system,
            path: [cave_system["start"].iter().peekable()].into(),
            passes: HashMap::from_iter(
                cave_system
                    .keys()
                    .filter(|&&cave| is_small_cave(cave))
                    .map(|&k| (k, false)),
            ),
            extra_pass: (!extra_pass).then(|| "start"),
        }
    }

    fn next(&mut self) -> bool {
        loop {
            if let Some(layer) = self.path.last_mut() {
                match layer.peek() {
                    Some(&&"end") => {
                        self.debug();
                        self.path.last_mut().unwrap().next();
                        return true;
                    }
                    None => {
                        self.path.pop();
                        if let Some(layer) = self.path.last_mut() {
                            let cave = **layer.peek().unwrap();
                            layer.next();
                            assert!(self.remove_pass(cave));
                        }
                    }
                    Some(&&cave) => {
                        if self.insert_pass(cave) {
                            self.path.push(self.cave_system[cave].iter().peekable());
                        } else {
                            self.path.last_mut().unwrap().next();
                        }
                    }
                }
            } else {
                return false;
            }
        }
    }

    fn insert_pass(&mut self, cave: &'a str) -> bool {
        if let Some(pass) = self.passes.get_mut(cave) {
            !mem::replace(pass, true)
                || self.extra_pass.is_none() && {
                    self.extra_pass = Some(cave);
                    true
                }
        } else {
            true
        }
    }

    fn remove_pass(&mut self, cave: &'a str) -> bool {
        if self.extra_pass == Some(cave) {
            self.extra_pass = None;
            true
        } else if let Some(pass) = self.passes.get_mut(cave) {
            mem::replace(pass, false)
        } else {
            true
        }
    }

    fn debug(&mut self) {
        print!("path: ");
        for layer in &mut self.path {
            print!("{},", layer.peek().map_or("None", |c| **c));
        }
        println!();
        println!("{:?}", self.passes);
    }
}
