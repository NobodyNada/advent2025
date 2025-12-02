pub use aoc_2025::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c
        .input
        .split(b',')
        .map(Result::unwrap)
        .map(String::from_utf8)
        .map(Result::unwrap);

    let mut sum = 0;
    for range in input {
        let (l, r) = range.trim_ascii().split_once('-').unwrap();
        let (l, r) = (l.parse::<u64>()?, r.parse::<u64>()?);

        for id in l..=r {
            let s = id.to_string();
            let max_substr_len = s.len().div_ceil(2);
            let min_substr_len = if c.part1 { max_substr_len } else { 1 };
            for substr_len in min_substr_len..=max_substr_len {
                if is_match(&s, substr_len) {
                    sum += id;
                    break;
                }
            }
        }
    }

    println!("{sum}");

    Ok(())
}

fn is_match(s: &str, substr_len: usize) -> bool {
    if s.len() == substr_len || !s.len().is_multiple_of(substr_len) {
        return false;
    }
    for offset in (substr_len..s.len()).step_by(substr_len) {
        if s[..substr_len] != s[offset..][..substr_len] {
            return false;
        }
    }
    true
}
