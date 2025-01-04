use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Pos(isize, isize);

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;
    fn add(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Pos(self.0, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 + 1),
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Player {
    dir: Direction,
    pos: Pos,
}

impl Player {
    fn step(&mut self) {
        self.pos = self.peek_step();
    }

    fn peek_step(&self) -> Pos {
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
    blocks: Vec<Pos>,
    width: isize,
    height: isize,
}

impl World {
    pub fn new(s: impl AsRef<str>) -> Self {
        let mut player = Player {
            dir: Direction::Up,
            pos: Pos(0, 0),
        };
        let mut blocks: Vec<Pos> = Vec::new();

        let (mut max_width, mut height) = (0, 0);
        for line in s.as_ref().lines() {
            let mut width = 0;
            for ch in line.chars() {
                match ch {
                    '#' => blocks.push(Pos(width, height)),
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

    fn out_of_bounds(&self, pos: &Pos) -> bool {
        pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height
    }

    fn steps(&mut self) -> Stepper {
        Stepper { world: self }
    }

    fn is_infinite_looping(&self) -> bool {
        let mut speculative_world = self.clone();
        let mut player_states: HashSet<Player> = HashSet::new();
        for step in speculative_world.steps() {
            if player_states.contains(&step) {
                return true;
            }
            player_states.insert(step);
        }
        false
    }
}

struct Stepper<'a> {
    world: &'a mut World,
}

impl<'a> Iterator for Stepper<'a> {
    type Item = Player;

    fn next(&mut self) -> Option<Player> {
        if self.world.out_of_bounds(&self.world.player.pos) {
            return None;
        }

        let result = self.world.player.clone();
        let next_pos = self.world.player.peek_step();

        // If next_pos hits a block, instead turn and try again.
        if self.world.blocks.contains(&next_pos) {
            self.world.player.turn();
            return self.next();
        }

        // Otherwise, move the player.
        self.world.player.step();
        Some(result)
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
                ],
            })
        )
    }

    #[gtest]
    fn test_stepping() -> Result<()> {
        let mut world = World::new(DATA);
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
        let mut world = World::new(DATA);
        let steps = world.steps();
        let posn: HashSet<_> = steps.map(|player| player.pos).collect();
        verify_that!(posn.len(), eq(41))
    }

    #[gtest]
    fn test_infinite_looping_negative() -> Result<()> {
        let world = World::new(DATA);
        verify_that!(world.is_infinite_looping(), is_false())
    }

    #[gtest]
    fn test_infinite_looping_positive() -> Result<()> {
        let mut world = World::new(DATA);
        world.blocks.push(Pos(3, 6));
        verify_that!(world.is_infinite_looping(), is_true())
    }

    #[gtest]
    fn test_part2() -> Result<()> {
        let world = World::new(DATA);
        verify_that!(part_2(&world), eq(6))
    }
}

fn part_1(world: &World) -> usize {
    let mut world = world.clone();
    let steps = world.steps();
    let posn: HashSet<_> = steps.collect();
    posn.len()
}

fn part_2(world: &World) -> usize {
    let states_to_check: HashSet<Pos> = {
        let mut world = world.clone();
        let mut steps = world.steps();
        let _ = steps.next();
        // Ignoring the first, see if placing a barrier causes an infinite loop.
        steps.map(|player| player.pos).collect()
    };

    let mut world = world.clone();
    let mut count = 0;
    for pos in states_to_check {
        world.blocks.push(pos);

        if world.is_infinite_looping() {
            count += 1;
        }
        world.blocks.pop();
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
