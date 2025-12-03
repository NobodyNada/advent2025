pub use aoc_2025::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c
        .input
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty());

    Ok(())
}
