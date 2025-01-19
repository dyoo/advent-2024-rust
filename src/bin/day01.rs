use std::collections::HashMap;
use std::error::Error;
use std::io;

fn distance(xs: impl IntoIterator<Item = u32>, ys: impl IntoIterator<Item = u32>) -> u32 {
    let mut xs: Vec<_> = xs.into_iter().collect();
    let mut ys: Vec<_> = ys.into_iter().collect();
    xs.sort();
    ys.sort();
    xs.into_iter().zip(ys).map(|(x, y)| x.abs_diff(y)).sum()
}

fn similarity(xs: impl IntoIterator<Item = u32>, ys: impl IntoIterator<Item = u32>) -> u32 {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for y in ys {
        *counts.entry(y).or_default() += 1;
    }
    xs.into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_distance() -> Result<()> {
        let xs = [3, 4, 2, 1, 3, 3];
        let ys = [4, 3, 5, 3, 9, 3];
        verify_that!(distance(xs, ys), eq(11))
    }

    #[gtest]
    fn test_similarity() -> Result<()> {
        let xs = [3, 4, 2, 1, 3, 3];
        let ys = [4, 3, 5, 3, 9, 3];
        verify_that!(similarity(xs, ys), eq(31))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();
    for (lineno, line) in io::stdin().lines().enumerate() {
        let line = line?;
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;
        lhs.push(
            numbers
                .get(0)
                .copied()
                .ok_or_else(|| format!("Missing lhs on line {}", lineno))?,
        );
        rhs.push(
            numbers
                .get(1)
                .copied()
                .ok_or_else(|| format!("Missing rhs on line {}", lineno))?,
        );
    }

    println!(
        "Distance: {}",
        distance(lhs.iter().copied(), rhs.iter().copied())
    );
    println!("Similarity: {}", similarity(lhs, rhs));

    Ok(())
}
