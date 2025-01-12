#[derive(Debug, PartialEq)]
struct Field {
    //    data: Box<[Box<[char]>]>,
    antennas: Box<[Antenna]>,
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
        }
    }
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    println!("Hello, world!");
    Ok(())
}
