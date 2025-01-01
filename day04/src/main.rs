use std::error::Error;
use std::io;

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
        Self { body }
    }

    pub fn streak(
        &self,
        initial_row: usize,
        initial_col: usize,
        delta_row: isize,
        delta_col: isize,
        len: usize,
    ) -> Streak<'_> {
        Streak {
            field: self,
            row: initial_row,
            col: initial_col,
            delta_row,
            delta_col,
            len,
        }
    }
}

struct Streak<'a> {
    field: &'a Field,
    row: usize,
    col: usize,
    delta_row: isize,
    delta_col: isize,
    len: usize,
}

impl<'a> Iterator for Streak<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let row = self.field.body.get(self.row)?;
        let result = row.get(self.col)?;
        if let Some(next_row) = self.row.checked_add_signed(self.delta_row) {
            self.row = next_row;
        }
        if let Some(next_col) = self.col.checked_add_signed(self.delta_col) {
            self.col = next_col;
        }
        self.len -= 1;

        Some(*result)
    }
}

fn count_xmas(field: &Field) -> u32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let field = Field::new(io::read_to_string(io::stdin())?);

    println!("Part 1: {}", count_xmas(&field));
    Ok(())
}
