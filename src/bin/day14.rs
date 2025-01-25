#[derive(Debug, PartialEq, Copy, Clone)]
struct Point(i32, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn simulate_movement(self, n: u32, width: i32, height: i32) -> Self {
        Self {
            pos: (self.pos + (n as i32) * self.vel).modulate(width, height),
            vel: self.vel,
        }
    }
}

impl Point {
    fn modulate(self, width: i32, height: i32) -> Self {
        Self(self.0.rem_euclid(width), self.1.rem_euclid(height))
    }
}

impl std::ops::Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, other: Point) -> Self::Output {
        Point(self * other.0, self * other.1)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_arithmetic() -> Result<()> {
        verify_that!(Point(0, 4) + 2 * Point(3, -3), eq(Point(6, -2)))
    }

    #[gtest]
    fn test_modulate() -> Result<()> {
        verify_that!(
            (Point(2, 4) + 5 * Point(2, -3)).modulate(11, 7),
            eq(Point(1, 3))
        )
    }
}

mod parser {
    use super::*;

    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, line_ending, space1};
    use nom::multi::{many1, separated_list0};
    use nom::IResult;

    pub fn parse_position(input: &str) -> IResult<&str, Point> {
        let (input, _) = tag("p=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i32(input)?;
        Ok((input, Point(x, y)))
    }

    pub fn parse_velocity(input: &str) -> IResult<&str, Point> {
        let (input, _) = tag("v=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i32(input)?;
        Ok((input, Point(x, y)))
    }

    pub fn parse_robot(input: &str) -> IResult<&str, Robot> {
        let (input, pos) = parse_position(input)?;
        let (input, _) = space1(input)?;
        let (input, vel) = parse_velocity(input)?;
        Ok((input, Robot { pos, vel }))
    }

    pub fn parse_all_robots(input: &str) -> IResult<&str, Vec<Robot>> {
        separated_list0(many1(line_ending), parse_robot)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use googletest::prelude::*;

        #[gtest]
        fn test_parse_robot() -> Result<()> {
            verify_that!(
                parse_robot("p=0,4 v=3,-3")?,
                eq((
                    "",
                    Robot {
                        pos: Point(0, 4),
                        vel: Point(3, -3)
                    }
                ))
            )
        }
    }
}

fn part_1(robots: Vec<Robot>) -> u32 {
    let robots: Vec<Robot> = robots
        .into_iter()
        .map(|r| r.simulate_movement(100, 101, 103))
        .collect();
    let mut scores = Vec::new();
    for col_range in [0..50, 51..101] {
        for row_range in [0..51, 52..103] {
            scores.push(
                robots
                    .iter()
                    .filter(|r| col_range.contains(&r.pos.0) && row_range.contains(&r.pos.1))
                    .count() as u32,
            );
        }
    }
    scores.into_iter().product()
}

/// Exploration to find some kind of interesting pattern.
fn visualize(robots: &[Robot], width: usize, height: usize) -> bool {
    let mut buffer = vec![vec!['.'; width]; height];
    for r in robots {
        buffer[r.pos.1 as usize][r.pos.0 as usize] = '*';
    }

    let mut possible_match = false;
    for line in buffer.iter() {
        let line = line.iter().collect::<String>();
        if line.contains("*************") {
            possible_match = true;
        }
    }

    if !possible_match {
        return false;
    }
    for line in buffer.iter() {
        let line = line.iter().collect::<String>();
        println!("{}", line);
    }
    true
}

fn part_2(mut robots: Vec<Robot>) {
    for i in 0..10000 {
        if visualize(&robots, 101, 103) {
            println!("{}", i);
            println!();
        }

        robots = robots
            .into_iter()
            .map(|r| r.simulate_movement(1, 101, 103))
            .collect();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, robots) = parser::parse_all_robots(&std::io::read_to_string(std::io::stdin())?)
        .map_err(|e| e.to_owned())?;
    println!("{:?}", part_1(robots.clone()));

    part_2(robots);
    Ok(())
}
