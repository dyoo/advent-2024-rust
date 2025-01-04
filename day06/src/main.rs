use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Pos(u32, u32);

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
    /// Tentatively walk forward, within bounds.  If we go out of bounds, None.
    fn peek_step(&self, width: u32, height: u32) -> Option<Pos> {
        (self.pos + self.dir).filter(|pos| pos.0 < width && pos.1 < height)
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
struct FieldMap {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl FieldMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn insert(&mut self, p: &Pos) {
        let index = self.width * p.1 as usize + p.0 as usize;
        self.data[index] = true;
    }

    fn remove(&mut self, p: &Pos) {
        let index = self.width * p.1 as usize + p.0 as usize;
        self.data[index] = false;
    }

    fn contains(&self, p: &Pos) -> bool {
        let index = self.width * p.1 as usize + p.0 as usize;
        self.data[index]
    }
}

#[derive(Debug, PartialEq, Clone)]
struct World {
    player: Player,
    field_map: FieldMap,
    width: u32,
    height: u32,
}

impl World {
    pub fn new(s: impl AsRef<str>) -> Self {
        let mut player = Player {
            dir: Direction::Up,
            pos: Pos(0, 0),
        };
        let mut positions = Vec::new();

        let (mut max_width, mut height) = (0, 0);
        for line in s.as_ref().lines() {
            let mut width = 0;
            for ch in line.chars() {
                match ch {
                    '#' => {
                        positions.push(Pos(width, height));
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

        let mut field_map = FieldMap::new(max_width as usize, height as usize);
        for pos in positions {
            field_map.insert(&pos);
        }

        World {
            player,
            field_map,
            width: max_width,
            height,
        }
    }

    fn steps(&self) -> Stepper<'_> {
        Stepper {
            field_map: &self.field_map,
            player: self.player.clone(),
            exhausted: false,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Clone)]
struct Stepper<'a> {
    field_map: &'a FieldMap,
    player: Player,
    exhausted: bool,
    width: u32,
    height: u32,
}

impl<'a> Stepper<'a> {
    fn peek(&mut self) -> Option<Player> {
        if self.exhausted {
            return None;
        }
        Some(self.player.clone())
    }

    fn is_infinite_looping(&self) -> bool {
        let mut player_states: HashSet<Player> = HashSet::new();
        let mut last_pos: Option<Pos> = None;
        for step in self.clone() {
            match last_pos {
                Some(pos) if pos == step.pos => {
                    if player_states.contains(&step) {
                        return true;
                    }
                    player_states.insert(step.clone());
                }
                _ => {}
            }
            last_pos = Some(step.pos);
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

        let Some(next_pos) = self.player.peek_step(self.width, self.height) else {
            // Out of bounds.  Mark this.
            self.exhausted = true;
            return result;
        };

        // If next_pos hits a block, instead turn.
        if self.field_map.contains(&next_pos) {
            self.player.turn();
            return result;
        }

        // Otherwise, move the player forward.
        self.player.pos = next_pos;
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
        let mut field_map = FieldMap::new(10, 10);
        for pos in [
            Pos(4, 0),
            Pos(9, 1),
            Pos(2, 3),
            Pos(7, 4),
            Pos(1, 6),
            Pos(8, 7),
            Pos(0, 8),
            Pos(6, 9),
        ] {
            field_map.insert(&pos);
        }
        verify_that!(
            world,
            eq(&World {
                width: 10,
                height: 10,
                player: Player {
                    pos: Pos(4, 6),
                    dir: Direction::Up
                },
                field_map,
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
        world.field_map.insert(&Pos(3, 6));
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
    let mut field_map = world.field_map.clone();

    let mut visited = FieldMap::new(world.width as usize, world.height as usize);

    for step_ahead in steps_ahead {
        if !visited.contains(&step_ahead.pos) {
            field_map.insert(&step_ahead.pos);

            let speculative_steps = Stepper {
                field_map: &field_map,
                ..steps.clone()
            };
            if speculative_steps.is_infinite_looping() {
                count += 1;
            }

            field_map.remove(&step_ahead.pos);
            visited.insert(&step_ahead.pos);
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
