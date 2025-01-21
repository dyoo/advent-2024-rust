#![allow(dead_code)]

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u64};
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::error::Error;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Point(u64, u64);

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
fn dijkstra_solver(a: &Point, b: &Point, prize: &Point) -> Option<u64> {
    let mut heap = BinaryHeap::new();

    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
    struct State {
        tokens: u64,
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
    fn test_dijkstra_solver_small() -> Result<()> {
        verify_that!(
            dijkstra_solver(&Point(94, 34), &Point(22, 67), &Point(8400, 5400)),
            some(eq(280))
        )?;

        verify_that!(
            dijkstra_solver(&Point(26, 66), &Point(67, 21), &Point(12748, 12176)),
            none()
        )?;

        verify_that!(
            dijkstra_solver(&Point(17, 86), &Point(84, 37), &Point(7870, 6450)),
            some(eq(200))
        )?;

        verify_that!(
            dijkstra_solver(&Point(64, 23), &Point(27, 71), &Point(18641, 10279)),
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

    #[gtest]
    fn test_parse_claw() -> Result<()> {
        let (_, (a, b, prize)) = parse_claw(
            "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        )?;
        verify_that!(a, eq(Point(94, 34)))?;
        verify_that!(b, eq(Point(22, 67)))?;
        verify_that!(prize, eq(Point(8400, 5400)))?;

        Ok(())
    }
}

fn parse_button(input: &str) -> IResult<&str, (&str, Point)> {
    let (input, _) = tag("Button ")(input)?;
    // eat A or B
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": X+")(input)?;
    let (input, x) = u64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = u64(input)?;

    Ok((input, (name, Point(x, y))))
}

fn parse_prize(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = u64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = u64(input)?;
    Ok((input, Point(x, y)))
}

fn parse_claw(input: &str) -> IResult<&str, (Point, Point, Point)> {
    let (input, (_, a)) = parse_button(input)?;
    let (input, _) = line_ending(input)?;
    let (input, (_, b)) = parse_button(input)?;
    let (input, _) = line_ending(input)?;
    let (input, prize) = parse_prize(input)?;
    Ok((input, (a, b, prize)))
}

fn parse_all_claws(s: &str) -> IResult<&str, Vec<(Point, Point, Point)>> {
    separated_list0(many1(line_ending), parse_claw)(s)
}

fn part_1(claws: &[(Point, Point, Point)]) -> u64 {
    claws
        .into_iter()
        .filter_map(|(a, b, prize)| dijkstra_solver(a, b, prize))
        .sum()
}

fn part_2(claws: &[(Point, Point, Point)]) -> u64 {
    claws
        .into_iter()
        .map(|(a, b, prize)| {
            (
                a,
                b,
                Point(prize.0 + 10000000000000, prize.1 + 10000000000000),
            )
        })
        .filter_map(|(a, b, prize)| dijkstra_solver(a, b, &prize))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let (input, claws) = parse_all_claws(&input).map_err(|e| e.to_owned())?;

    println!("Part 1: {}", part_1(&claws));

    // Essentially, we're trying to find naturals n1, n2 such that
    //    n1 * A + n2 * B = prize
    // and
    //    Cost(n1, n2) = 3*n1 + n2 is minimized.
    //
    // Can we treat this algebraically as a calculus problem?
    //
    // We can express n2 in terms of n1 for each problem, because
    //
    // n1 * A + n2 * B = prize
    // ==>  n2 * B = (prize - n1 * A)

    // Cost(n1, n2) * B = 3 * n1 * B + n2 * B
    // ==> Cost(n1) * B = 3 * n1 * B + (prize - n1 * A)

    println!("Part 2: {}", part_2(&claws));
    Ok(())
}
