#![feature(array_windows)]

use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

fn main() -> Result<()> {
    let f = io::BufReader::new(fs::File::open("day-1/input.txt")?);

    let depths = f
        .lines()
        .map(|line| -> Result<_> { Ok(line?.trim().parse::<u32>()?) })
        .collect::<Result<Vec<_>>>()?;
    let local_sums: Vec<_> = depths.array_windows().map(|&[a, b, c]| a + b + c).collect();
    let increases = local_sums
        .array_windows()
        .filter(|&[cur, next]| cur < next)
        .count();

    println!("The answer is: {}", increases);

    Ok(())
}
