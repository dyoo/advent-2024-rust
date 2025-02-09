#![allow(dead_code)]

use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    W,
    U,
    B,
    R,
    G,
}

impl TryFrom<char> for Color {
    type Error = Box<dyn Error>;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Color::W),
            'u' => Ok(Color::U),
            'b' => Ok(Color::B),
            'r' => Ok(Color::R),
            'g' => Ok(Color::G),
            _ => Err(format!("Unknown color {}", value).into()),
        }
    }
}

// A choice is a slice of colors.
type Choice = Box<[Color]>;

fn parse_choice(s: &str) -> Result<Choice, Box<dyn Error>> {
    s.chars().map(Color::try_from).collect()
}

fn parse_choices(s: &str) -> Result<Box<[Choice]>, Box<dyn Error>> {
    s.split(", ").map(parse_choice).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_parse_choices() -> Result<()> {
        let s = "r, wr, b, g, bwu, rb, gb, br";
        verify_that!(
            parse_choices(s).into_test_result()?,
            elements_are![
              container_eq([Color::R].into()),
              container_eq([Color::W, Color::R].into()),
              container_eq([Color::B].into()),
              container_eq([Color::G].into()),
              container_eq([Color::B, Color::W, Color::U].into()),
              container_eq([Color::R, Color::B].into()),
              container_eq([Color::G, Color::B].into()),
              container_eq([Color::B, Color::R].into()),
            ]
        )?;
        Ok(())
    }
}

fn main() {}
