fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let depths: Vec<u32> = INPUT
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let local_sums: Vec<u32> = depths.windows(3).map(|w| w.iter().sum()).collect();
    let increases = local_sums.windows(2).filter(|w| w[0] < w[1]).count();

    println!("The answer is: {}", increases);
}
