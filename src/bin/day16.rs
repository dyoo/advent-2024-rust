use advent_2024::{Direction, TileIndex};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
struct Maze {
    collision_map: Box<[bool]>, // we want this repr for cheap cloning.
    tiles: TileIndex,
    goal: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
struct PlayerState {
    pos: usize,
    dir: Direction,
}

/// Find minimal score navigating the maze.
fn search(maze: &Maze, start: &PlayerState) -> Option<u32> {
    let mut heap: BinaryHeap<Reverse<(u32, PlayerState)>> = BinaryHeap::new();
    let mut visited: HashSet<PlayerState> = HashSet::new();
    heap.push(Reverse((0, start.clone())));

    while let Some(Reverse((score, player))) = heap.pop() {
        if visited.contains(&player) {
            continue;
        }
        visited.insert(player.clone());

        if player.pos == maze.goal {
            return Some(score);
        }

        if let Some(p) = player.forward(maze) {
            heap.push(Reverse((score + 1, p)));
        }
        heap.push(Reverse((score + 1000, player.clock())));
        heap.push(Reverse((score + 1000, player.counterclock())));
    }

    None
}

fn parse(s: &str) -> (Maze, PlayerState) {
    let lines = s.trim().lines();
    let chars = lines.clone().flat_map(|line| line.trim().chars());
    let height = lines.count();

    let collision_map: Vec<bool> = chars.clone().map(|ch| ch == '#').collect();
    let width = collision_map.iter().count() / height;
    let pos = chars.clone().position(|ch| ch == 'S').expect("Start");

    let mut chars = chars;
    let goal = chars.position(|ch| ch == 'E').expect("End");

    (
        Maze {
            collision_map: collision_map.into(),
            tiles: TileIndex { width, height },
            goal,
        },
        PlayerState {
            pos,
            dir: Direction::Right,
        },
    )
}

impl PlayerState {
    // Try to move forward if we don't collide with a wall.
    fn forward(&self, maze: &Maze) -> Option<Self> {
        let Some(new_pos) = maze.tiles.dir_to(self.pos, self.dir) else {
            return None;
        };
        if maze.collision_map[new_pos] {
            None
        } else {
            Some(Self {
                pos: new_pos,
                ..*self
            })
        }
    }

    fn clock(&self) -> Self {
        Self {
            dir: self.dir.clock(),
            ..*self
        }
    }

    fn counterclock(&self) -> Self {
        Self {
            dir: self.dir.counterclock(),
            ..*self
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (maze, player) = parse(&std::io::read_to_string(std::io::stdin())?);
    println!("Part 1: {:?}", search(&maze, &player));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const data: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[gtest]
    fn test_parse() -> Result<()> {
        let (maze, player) = parse(
            "
###############
#.......#....E#
#S..#.....#...#
###############
",
        );
        verify_that!(maze.tiles.height, eq(4))?;
        verify_that!(maze.tiles.width, eq(15))?;
        verify_that!(maze.goal, eq(28))?;
        verify_that!(player.pos, eq(31))?;

        Ok(())
    }

    #[gtest]
    fn test_search() -> Result<()> {
        let (maze, player) = parse(data);
        verify_that!(search(&maze, &player), some(eq(7036)))
    }
}
