pub use aoc_2025::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c
        .input
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty());

    let mut pos = 50;
    let mut password = 0;
    for line in input {
        let prev = pos;
        let (dir, n) = line.split_at(1);
        let n = n.parse::<i32>().unwrap();
        match dir {
            "L" => pos -= n,
            "R" => pos += n,
            _ => panic!("invalid input: {line}"),
        }
        if c.part1 {
            pos = pos.rem_euclid(100);
            if pos == 0 {
                password += 1;
            }
        } else {
            print!("{dir}{n}\t");
            #[allow(clippy::collapsible_if)]
            if pos > 0 {
                password += pos / 100;
                pos %= 100;
            } else if pos == 0 && n != 0 {
                password += 1;
            } else if pos < 0 {
                if prev != 0 {
                    password += 1;
                }
                password += pos.abs() / 100;
                pos = pos.rem_euclid(100);
            }
            println!("{pos} {password}");
        }
    }

    println!("{password}");

    Ok(())
}
