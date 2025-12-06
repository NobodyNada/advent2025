use std::ops::RangeInclusive;

pub use aoc_2025::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap);
    let mut ranges = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|range| *range.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut()
            && last.end() >= range.start()
        {
            *last = *last.start()..=*last.end().max(range.end())
        } else {
            merged.push(range);
        }
    }
    let ranges = merged;

    if c.part1 {
        let count = input
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|item| ranges.iter().any(|range| range.contains(item)))
            .count();
        println!("{count}");
    } else {
        let count: u64 = ranges.into_iter().map(|range| range.count() as u64).sum();
        println!("{count}");
    }

    Ok(())
}
