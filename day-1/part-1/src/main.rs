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
    let increases = depths
        .array_windows()
        .filter(|&[first, next]| first < next)
        .count();

    println!("The answer is: {}", increases);

    Ok(())
}
