use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Field {
    antennas: Box<[Antenna]>,
    rows: isize,
    cols: isize,
}

#[derive(Debug, PartialEq)]
struct Antenna {
    label: char,
    row: isize,
    col: isize,
}

impl Field {
    pub fn parse(s: &str) -> Self {
        let data: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let mut antennas = vec![];
        for (row, line) in data.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                match ch {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                        let antenna = Antenna {
                            label: *ch,
                            row: row as isize,
                            col: col as isize,
                        };
                        antennas.push(antenna);
                    }
                    _ => {}
                }
            }
        }

        Self {
            antennas: antennas.into(),
            rows: data.len() as isize,
            cols: data[0].len() as isize,
        }
    }

    fn in_bounds(&self, pos: &(isize, isize)) -> bool {
        0 <= pos.0 && pos.0 < self.rows && 0 <= pos.1 && pos.1 < self.cols
    }

    pub fn antinodes(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.antennas.iter().flat_map(|from| {
            self.antennas
                .iter()
                .filter(|to| from.label == to.label && (from.row != to.row || from.col != to.col))
                .flat_map(|to| Some(from.antinode(to)).filter(|pos| self.in_bounds(pos)))
        })
    }

    pub fn line_antinodes(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.antennas.iter().flat_map(|from| {
            self.antennas
                .iter()
                .filter(|to| from.label == to.label && (from.row != to.row || from.col != to.col))
                .flat_map(|to| {
                    LineAntinode::new((from.row, from.col), (to.row, to.col))
                        .take_while(|pos| self.in_bounds(pos))
                })
        })
    }
}

struct LineAntinode {
    pos: (isize, isize),
    delta_row: isize,
    delta_col: isize,
}

impl LineAntinode {
    fn new(from: (isize, isize), to: (isize, isize)) -> Self {
        let (delta_row, delta_col) = (to.0 - from.0, to.1 - from.1);
        LineAntinode {
            pos: (to.0, to.1),
            delta_row,
            delta_col,
        }
    }
}

impl Iterator for LineAntinode {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.pos;
        self.pos = (self.pos.0 + self.delta_row, self.pos.1 + self.delta_col);
        Some(next_result)
    }
}

impl Antenna {
    fn antinode(&self, other: &Antenna) -> (isize, isize) {
        let (delta_row, delta_col) = (other.row - self.row, other.col - self.col);
        (other.row + delta_row, other.col + delta_col)
    }
}

fn part_1(field: &Field) -> usize {
    let unique_locations: HashSet<_> = field.antinodes().collect();
    unique_locations.len()
}

fn part_2(field: &Field) -> usize {
    let unique_locations: HashSet<_> = field.line_antinodes().collect();
    unique_locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const DATA: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[gtest]
    fn test_parse() -> Result<()> {
        let field = Field::parse(DATA);
        verify_that!(
            field.antennas,
            unordered_elements_are!(
                eq(&Antenna {
                    label: '0',
                    row: 1,
                    col: 8
                }),
                eq(&Antenna {
                    label: '0',
                    row: 2,
                    col: 5
                }),
                eq(&Antenna {
                    label: '0',
                    row: 3,
                    col: 7
                }),
                eq(&Antenna {
                    label: '0',
                    row: 4,
                    col: 4
                }),
                eq(&Antenna {
                    label: 'A',
                    row: 5,
                    col: 6
                }),
                eq(&Antenna {
                    label: 'A',
                    row: 8,
                    col: 8
                }),
                eq(&Antenna {
                    label: 'A',
                    row: 9,
                    col: 9
                }),
            )
        )
    }

    #[gtest]
    fn test_part1() -> Result<()> {
        let field = Field::parse(DATA);
        verify_that!(part_1(&field), eq(14))
    }

    #[gtest]
    fn test_part2() -> Result<()> {
        let field = Field::parse(DATA);
        verify_that!(part_2(&field), eq(34))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let field = Field::parse(&input);
    println!("Part 1: {}", part_1(&field));
    println!("Part 2: {}", part_2(&field));
    Ok(())
}
