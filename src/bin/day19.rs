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
type ColorString = Box<[Color]>;

fn parse_color_string(s: &str) -> Result<ColorString, Box<dyn Error>> {
    s.chars().map(Color::try_from).collect()
}

fn parse_choices(s: &str) -> Result<Box<[ColorString]>, Box<dyn Error>> {
    s.split(", ").map(parse_color_string).collect()
}

#[derive(Debug, PartialEq)]
struct Problem {
    choices: Box<[ColorString]>,
    designs: Box<[ColorString]>,
}

fn parse_problem(s: &str) -> Result<Problem, Box<dyn Error>> {
    let mut items = s.split("\n\n");
    let choices = items
        .next()
        .map(parse_choices)
        .ok_or::<Box<dyn Error>>("Missing choices".into())??;
    let designs = items
        .next()
        .ok_or::<Box<dyn Error>>("Missing designs".into())?
        .lines()
        .map(parse_color_string)
        .collect::<Result<Vec<_>, _>>()?
        .into();
    Ok(Problem { choices, designs })
}

fn is_possible(choices: &[ColorString], pattern: &[Color]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for choice in choices {
        if choice.len() > pattern.len() {
            continue;
        }

        if pattern[..choice.len()] == choice[..] && is_possible(choices, &pattern[choice.len()..]) {
            return true;
        }
    }

    false
}

fn count_possibles(choices: &[ColorString], pattern: &[Color]) -> u64 {
    let mut suffix_cache = vec![0; pattern.len() + 1];
    suffix_cache[pattern.len()] = 1;

    // Work backwards
    for i in (0..pattern.len()).rev() {
        for choice in choices {
            if i + choice.len() > pattern.len() {
                continue;
            }

            if choice[..] == pattern[i..i + choice.len()] {
                suffix_cache[i] += suffix_cache[i + choice.len()];
            }
        }
    }
    // Final count should be here:
    suffix_cache[0]
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

fn main() -> Result<(), Box<dyn Error>> {
    let problem = parse_problem(&std::io::read_to_string(std::io::stdin())?)?;

    println!(
        "Part 1: {}",
        problem
            .designs
            .iter()
            .filter(|design| is_possible(&problem.choices, design))
            .count(),
    );

    println!(
        "Part 2: {}",
        problem
            .designs
            .iter()
            .map(|design| count_possibles(
                &problem.choices,
                &design[..],
            ))
            .sum::<u64>(),
    );

    Ok(())
}
