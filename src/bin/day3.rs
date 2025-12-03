pub use aoc_2025::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c
        .input
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty());

    let sum: u64 = input
        .map(|line| joltage(&line, if c.part1 { 2 } else { 12 }, 0))
        .sum();

    println!("{sum}");

    Ok(())
}

fn joltage(line: &str, n: usize, current: u64) -> u64 {
    if n == 0 {
        current
    } else {
        let digits = line.as_bytes()[..=line.len() - n]
            .iter()
            .map(|b| (*b as char).to_digit(10).unwrap());
        let (idx, max) = digits
            .enumerate()
            .rev()
            .max_by_key(|(_, digit)| *digit)
            .unwrap();
        joltage(&line[idx + 1..], n - 1, current * 10 + max as u64)
    }
}
