use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::io::read_to_string(std::io::stdin())?;
    let data: Vec<Vec<u32>> = parse(&content)?;
    println!("Part 1: {:?}", data.iter().filter(|&v| is_safe(v)).count());
    println!(
        "Part 2: {:?}",
        data.iter().filter(|&v| is_almost_safe(v)).count()
    );
    Ok(())
}

fn is_safe(row: &[u32]) -> bool {
    (all_pairwise(row, |x, y| x > y) || all_pairwise(row, |x, y| x < y))
        && all_pairwise(row, |x, y| {
            let diff = x.abs_diff(y);
            (1..=3).contains(&diff)
        })
}

fn is_almost_safe(row: &[u32]) -> bool {
    if is_safe(row) {
        return true;
    }
    for i in 0..row.len() {
        let mut modified = Vec::from(row);
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }
    false
}

struct Pairing<'a, T> {
    vals: &'a [T],
    index: usize,
}

impl<'a, T> Pairing<'a, T> {
    fn new(vals: &'a [T]) -> Self {
        Pairing { vals, index: 0 }
    }
}

impl<'a, T> Iterator for Pairing<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vals.len() - 1 {
            let result = (&self.vals[self.index], &self.vals[self.index + 1]);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn all_pairwise(row: &[u32], test: impl Fn(u32, u32) -> bool) -> bool {
    Pairing::new(row).all(|(v1, v2)| test(*v1, *v2))
}

fn parse(content: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()
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
