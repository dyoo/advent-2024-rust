use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Pos(u8, u8);

impl std::ops::Add<Direction> for Pos {
    type Output = Option<Pos>;
    fn add(mut self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => self.1 = self.1.checked_sub(1)?,
            Direction::Down => self.1 = self.1.checked_add(1)?,
            Direction::Left => self.0 = self.0.checked_sub(1)?,
            Direction::Right => self.0 = self.0.checked_add(1)?,
        }
        Some(self)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Player {
    dir: Direction,
    pos: Pos,
}

impl Player {
    fn step(&mut self) {
        self.pos = self.peek_step().unwrap();
    }

    fn peek_step(&self) -> Option<Pos> {
        self.pos + self.dir
    }

    fn turn(&mut self) {
        self.dir = self.dir.turn();
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(ch: char) -> Self {
        match ch {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'V' => Direction::Down,
            _ => panic!("Unknown direction {:?}", ch),
        }
    }

    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct World {
    player: Player,
    blocks: HashSet<Pos>,
    width: u8,
    height: u8,
}

impl World {
    pub fn new(s: impl AsRef<str>) -> Self {
        let mut player = Player {
            dir: Direction::Up,
            pos: Pos(0, 0),
        };
        let mut blocks: HashSet<Pos> = HashSet::new();

        let (mut max_width, mut height) = (0, 0);
        for line in s.as_ref().lines() {
            let mut width = 0;
            for ch in line.chars() {
                match ch {
                    '#' => {
                        blocks.insert(Pos(width, height));
                    }
                    '^' | 'V' | '<' | '>' => {
                        player = Player {
                            pos: Pos(width, height),
                            dir: Direction::new(ch),
                        }
                    }
                    '.' => {}
                    _ => {
                        println!("I don't know {}", ch);
                    }
                }
                width += 1;
            }
            height += 1;
            max_width = std::cmp::max(max_width, width);
        }

        World {
            player,
            blocks,
            width: max_width,
            height,
        }
    }

    fn steps(&self) -> Stepper<'_> {
        Stepper {
            blocks: &self.blocks,
            player: self.player.clone(),
            negative_pos: false,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Clone)]
struct Stepper<'a> {
    blocks: &'a HashSet<Pos>,
    player: Player,
    negative_pos: bool,
    width: u8,
    height: u8,
}

impl<'a> Stepper<'a> {
    fn out_of_bounds(&self, pos: &Pos) -> bool {
        pos.0 >= self.width || pos.1 >= self.height
    }

    fn peek(&mut self) -> Option<Player> {
        if self.negative_pos || self.out_of_bounds(&self.player.pos) {
            return None;
        }
        Some(self.player.clone())
    }

    fn is_infinite_looping(&self) -> bool {
        let mut player_states: HashSet<Player> = HashSet::new();
        for step in self.clone() {
            if player_states.contains(&step) {
                return true;
            }
            player_states.insert(step);
        }
        false
    }
}

impl<'a> Iterator for Stepper<'a> {
    type Item = Player;

    fn next(&mut self) -> Option<Player> {
        let result = self.peek();
        if result.is_none() {
            return result;
        }

        loop {
            let Some(next_pos) = self.player.peek_step() else {
                // Out of bounds.  Mark this.
                self.negative_pos = true;
                return result;
            };

            // If next_pos hits a block, instead turn and try again.
            if self.blocks.contains(&next_pos) {
                self.player.turn();
            } else {
                break;
            }
        }

        // Otherwise, move the player.
        self.player.step();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const DATA: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[gtest]
    fn test_parsing() -> Result<()> {
        let world = World::new(DATA);
        verify_that!(
            world,
            eq(&World {
                width: 10,
                height: 10,
                player: Player {
                    pos: Pos(4, 6),
                    dir: Direction::Up
                },
                blocks: vec![
                    Pos(4, 0),
                    Pos(9, 1),
                    Pos(2, 3),
                    Pos(7, 4),
                    Pos(1, 6),
                    Pos(8, 7),
                    Pos(0, 8),
                    Pos(6, 9),
                ]
                .into_iter()
                .collect(),
            })
        )
    }

    #[gtest]
    fn test_stepping() -> Result<()> {
        let world = World::new(DATA);
        let mut steps = world.steps();
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 6))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 5))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 4))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 3))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 2))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(4, 1))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(5, 1))))?;
        verify_that!(steps.next().map(|p| p.pos), some(eq(Pos(6, 1))))?;
        Ok(())
    }

    #[gtest]
    fn test_distinct_pathing() -> Result<()> {
        let world = World::new(DATA);
        let steps = world.steps();
        let posn: HashSet<_> = steps.map(|player| player.pos).collect();
        verify_that!(posn.len(), eq(41))
    }

    #[gtest]
    fn test_infinite_looping_negative() -> Result<()> {
        let world = World::new(DATA);
        verify_that!(world.steps().is_infinite_looping(), is_false())
    }

    #[gtest]
    fn test_infinite_looping_positive() -> Result<()> {
        let mut world = World::new(DATA);
        world.blocks.insert(Pos(3, 6));
        verify_that!(world.steps().is_infinite_looping(), is_true())
    }

    #[gtest]
    fn test_part2() -> Result<()> {
        let world = World::new(DATA);
        verify_that!(part_2(&world), eq(6))
    }
}

fn part_1(world: &World) -> usize {
    let steps = world.steps();
    let posn: HashSet<_> = steps.collect();
    posn.len()
}

fn part_2(world: &World) -> usize {
    let mut steps = world.steps();
    let mut steps_ahead = steps.clone();
    let _ = steps_ahead.next();

    let mut count = 0;
    let mut blocks = world.blocks.clone();

    let mut visited = HashSet::new();

    for step_ahead in steps_ahead {
        if !visited.contains(&step_ahead.pos) {
            blocks.insert(step_ahead.pos);

            let speculative_steps = Stepper {
                blocks: &blocks,
                ..steps.clone()
            };
            if speculative_steps.is_infinite_looping() {
                count += 1;
            }

            blocks.remove(&step_ahead.pos);
            visited.insert(step_ahead.pos);
        }

        let _ = steps.next();
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let world = World::new(input);
    println!("Part 1: {}", part_1(&world));
    println!("Part 2: {}", part_2(&world));

    Ok(())
}
