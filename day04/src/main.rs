struct Field {
    body: Vec<Vec<char>>,
}

impl Field {
    fn new<S: AsRef<str>>(s: S) -> Self {
        let body: Vec<Vec<_>> = s
            .as_ref()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        // TODO: check presence of a row & that each row is the same length.
        Self { body }
    }

    pub fn streak(
        &self,
        initial_row: usize,
        initial_col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> Streak<'_> {
        Streak {
            field: self,
            row: initial_row,
            col: initial_col,
            delta_row,
            delta_col,
        }
    }

    fn row_len(&self) -> usize {
        self.body.len()
    }
    fn col_len(&self) -> usize {
        self.body[0].len()
    }
}

struct Streak<'a> {
    field: &'a Field,
    row: usize,
    col: usize,
    delta_row: isize,
    delta_col: isize,
}

impl<'a> Iterator for Streak<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.field.body.get(self.row)?;
        let result = row.get(self.col)?;
        if let Some(next_row) = self.row.checked_add_signed(self.delta_row) {
            self.row = next_row;
        }
        if let Some(next_col) = self.col.checked_add_signed(self.delta_col) {
            self.col = next_col;
        }

        Some(*result)
    }
}

fn matches_xmas(field: &Field, row: usize, col: usize, delta_row: isize, delta_col: isize) -> bool {
    let mut streak = field.streak(row, col, delta_row, delta_col);
    for ch_to_check in "XMAS".chars() {
        match streak.next() {
            Some(ch) if ch == ch_to_check => {
                // Continue scanning
            }
            _ => return false,
        }
    }
    true
}

fn count_xmas(field: &Field) -> u32 {
    let mut count = 0;
    for row in 0..field.row_len() {
        for col in 0..field.col_len() {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    if matches_xmas(field, row, col, i, j) {
                        println!("row={row}, col={col}, i={i}, j={j}");
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const S: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[gtest]
    fn test_streak() -> Result<()> {
        let field = Field::new(S);
        let first_four: Vec<char> = field.streak(0, 5, -1, 1).take(4).collect();
        verify_that!(first_four, eq(&vec!['X', 'X', 'S', 'A']))
    }

    #[test]
    fn test_example() {
        let field = Field::new(S);
        assert_eq!(count_xmas(&field), 18);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let field = Field::new(std::io::read_to_string(std::io::stdin())?);

    println!("Part 1: {}", count_xmas(&field));
    Ok(())
}
