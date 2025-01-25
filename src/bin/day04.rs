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
            exhausted: false,
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
    exhausted: bool,
}

impl Iterator for Streak<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let row = self.field.body.get(self.row)?;
        let result = row.get(self.col)?;
        if let Some(next_row) = self.row.checked_add_signed(self.delta_row) {
            self.row = next_row;
        } else {
            self.exhausted = true;
        }
        if let Some(next_col) = self.col.checked_add_signed(self.delta_col) {
            self.col = next_col;
        } else {
            self.exhausted = true;
        }

        Some(*result)
    }
}

fn matches_xmas(field: &Field, row: usize, col: usize, delta_row: isize, delta_col: isize) -> bool {
    let streak = field.streak(row, col, delta_row, delta_col);
    matches_prefix("XMAS".chars(), streak)
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
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn matches_prefix<T: PartialEq>(
    prefix: impl IntoIterator<Item = T>,
    seq: impl IntoIterator<Item = T>,
) -> bool {
    let mut lhs = prefix.into_iter();
    let mut rhs = seq.into_iter();
    loop {
        match (lhs.next(), rhs.next()) {
            (Some(v1), Some(v2)) if v1 == v2 => {}
            (None, _) => return true,
            _ => return false,
        }
    }
}

fn matches_xmas2(field: &Field, row: usize, col: usize) -> bool {
    // Four patterns to check:
    //
    // M.S    S.M    S.S    M.M
    // .A.    .A.    .A.    .A.
    // M.S    S.M    M.M    S.S
    for prefix1 in ["MAS", "SAM"] {
        for prefix2 in ["MAS", "SAM"] {
            if matches_prefix(prefix1.chars(), field.streak(row, col, 1, 1))
                && matches_prefix(prefix2.chars(), field.streak(row, col + 2, 1, -1))
            {
                return true;
            }
        }
    }
    false
}

fn count_xmas2(field: &Field) -> u32 {
    let mut result = 0;
    for row in 0..field.row_len() {
        for col in 0..field.col_len() {
            if matches_xmas2(field, row, col) {
                result += 1;
            }
        }
    }
    result
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
        let first_four: Vec<char> = field.streak(0, 5, 1, -1).take(4).collect();
        verify_that!(first_four, eq(&vec!['X', 'X', 'S', 'A']))
    }

    #[test]
    fn test_example() -> Result<()> {
        let field = Field::new(S);
        verify_that!(count_xmas(&field), eq(18))
    }

    #[test]
    fn test_matches_xmas2() -> Result<()> {
        let field = Field::new(S);
        verify_that!(count_xmas2(&field), eq(9))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let field = Field::new(std::io::read_to_string(std::io::stdin())?);

    println!("Part 1: {}", count_xmas(&field));
    println!("Part 2: {}", count_xmas2(&field));
    Ok(())
}
