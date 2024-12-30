use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::io::read_to_string(std::io::stdin())?;
    let data: Vec<Vec<u32>> = parse(&content)?;
    println!(
        "Part 1: {:?}",
        data.into_iter().filter(|v| is_safe(v)).count()
    );
    Ok(())
}

fn is_safe(row: &[u32]) -> bool {
    (all_pairwise(row, |x, y| x > y) || all_pairwise(row, |x, y| x < y))
        && all_pairwise(row, |x, y| {
            let diff = x.abs_diff(y);
            diff >= 1 && diff <= 3
        })
}

fn all_pairwise(row: &[u32], test: impl Fn(u32, u32) -> bool) -> bool {
    for i in 0..(row.len() - 1) {
        if !test(row[i], row[i + 1]) {
            return false;
        }
    }
    true
}

fn parse(content: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<u32>, ParseIntError>>()
        })
        .collect::<Result<Vec<Vec<u32>>, ParseIntError>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const TEST_DATA: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[gtest]
    fn test_parsing() -> Result<()> {
        verify_that!(
            parse(TEST_DATA)?,
            eq(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ])
        )
    }
}
