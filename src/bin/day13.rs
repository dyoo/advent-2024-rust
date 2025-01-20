#![allow(dead_code)]

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::u32;
use nom::IResult;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::error::Error;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Point(u32, u32);

impl Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, other: &Point) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Point) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, other: &Point) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

/// Returns the minimal number of tokens needed, assuming it takes three
/// tokens for an `a`, and one token for a `b`.
///
/// Idea: use Dijkstra's search, with two possible actions in the state space:
/// * press button a once
/// * press button b repeatedly directly to the prize
///
/// The asymmetry here is intentional, to reduce the size of the state
/// space: we want to treat the sequence `[a pressed, b pressed, a
/// pressed]` the same as `[a pressed, a pressed, b pressed]`.  So we
/// design the possible actions so that we keep a canonical sequence,
/// given the order independence between the button presses.
fn solver(a: &Point, b: &Point, prize: &Point) -> Option<u32> {
    let mut heap = BinaryHeap::new();

    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
    struct State {
        tokens: u32,
        point: Point,
    }

    heap.push(Reverse(State {
        tokens: 0,
        point: Point(0, 0),
    }));
    while let Some(Reverse(State { tokens, point })) = heap.pop() {
        if point == *prize {
            return Some(tokens);
        }

        // Action 1: press A.
        let point_after_a = point + a;
        if point_after_a.0 <= prize.0 && point_after_a.1 <= prize.1 {
            heap.push(Reverse(State {
                tokens: tokens + 3,
                point: point_after_a,
            }));
        }

        // Action 2: press B repeatedly if that can get us to the
        // prize directly.  Use divisibility.
        let delta = *prize - point;
        if delta.0 % b.0 == 0 && delta.1 % b.1 == 0 && (delta.0 / b.0) == (delta.1 / b.1) {
            heap.push(Reverse(State {
                tokens: tokens + (delta.0 / b.0),
                point: *prize,
            }));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_solver_small() -> Result<()> {
        verify_that!(
            solver(&Point(94, 34), &Point(22, 67), &Point(8400, 5400)),
            some(eq(280))
        )?;

        verify_that!(
            solver(&Point(26, 66), &Point(67, 21), &Point(12748, 12176)),
            none()
        )?;

        verify_that!(
            solver(&Point(17, 86), &Point(84, 37), &Point(7870, 6450)),
            some(eq(200))
        )?;

        verify_that!(
            solver(&Point(64, 23), &Point(27, 71), &Point(18641, 10279)),
            none()
        )?;

        Ok(())
    }

    #[gtest]
    fn test_parse_button() -> Result<()> {
        let (_, button) = parse_button("Button A: X+21, Y+56")?;
        verify_that!(button, eq(("A", Point(21, 56))))?;

        let (_, button) = parse_button("Button B: X+59, Y+28")?;
        verify_that!(button, eq(("B", Point(59, 28))))?;
        Ok(())
    }

    #[gtest]
    fn test_parse_prize() -> Result<()> {
        let (_, prize) = parse_prize("Prize: X=3892, Y=3840")?;
        verify_that!(prize, eq(Point(3892, 3840)))?;

        Ok(())
    }
}

fn parse_button(input: &str) -> IResult<&str, (&str, Point)> {
    let (input, _) = tag("Button ")(input)?;
    // eat A or B
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": X+")(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = u32(input)?;

    Ok((input, (name, Point(x, y))))
}

fn parse_prize(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = u32(input)?;
    Ok((input, Point(x, y)))
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
