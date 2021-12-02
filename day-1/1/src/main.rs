fn main() {
    const INPUT: &'static str = include_str!("../../input.txt");

    let depths: Vec<u32> = INPUT
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let increases = depths.windows(2).filter(|w| w[0] < w[1]).count();

    println!("The answer is: {}", increases);
}
