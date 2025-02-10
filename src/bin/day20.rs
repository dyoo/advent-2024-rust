#![allow(dead_code, unused_variables, unused_imports)]

use advent_2024::{Direction, TileIndex, DIRECTIONS};

use std::error::Error;

struct Maze {
    data: Box<[char]>,
    width: usize,
    height: usize,
    tiles: TileIndex,
    start_pos: usize,
    end_pos: usize,
}

impl Maze {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines();
        let height = lines.clone().count();
        let data: Box<[char]> = lines.flat_map(|line| line.trim().chars()).collect();
        let width = data.len() / height;
        let start_pos = data
            .iter()
            .position(|ch| *ch == 'S')
            .expect("start position");
        let end_pos = data.iter().position(|ch| *ch == 'E').expect("end position");
        Self {
            data,
            width,
            height,
            tiles: TileIndex { width, height },
            start_pos,
            end_pos,
        }
    }

    fn costs(&self) -> Vec<u32> {
        let mut costs = vec![u32::MAX; self.data.len()];

        // First, BFS to get costs to get from start to end.
        let mut to_visit = vec![self.start_pos];
        let mut visited = vec![false; self.data.len()];
        let mut current_step = 0;
        while !to_visit.is_empty() {
            let mut next_to_visit = Vec::<usize>::new();
            for next_position in to_visit {
                if visited[next_position] {
                    continue;
                }

                visited[next_position] = true;
                costs[next_position] = current_step;

                for dir in DIRECTIONS {
                    next_to_visit.extend(
                        self.tiles
                            .dir_to(next_position, dir)
                            .filter(|neighbor| !visited[*neighbor] && self.data[*neighbor] != '#'),
                    );
                }
            }
            to_visit = next_to_visit;
            current_step += 1;
        }
        costs
    }

    fn dig(&self, pos: usize, dir: Direction) -> Option<u32> {
        // Drill a cheat, rerun-costs, return cost to the end position.  If we
        // can't drill successfully, None.
        let Some(hole) = self.tiles.dir_to(pos, dir) else {
            return None;
        };
        if self.data[pos] != '#' && self.data[hole] != '#' {
            return None;
        }

        let mut new_maze = Maze {
            data: self.data.clone(),
            tiles: self.tiles.clone(),
            ..*self
        };
        new_maze.data[pos] = '.';
        new_maze.data[hole] = '.';
        let updated_costs = new_maze.costs();
        Some(updated_costs[self.end_pos])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_dig() -> Result<()> {
        let data = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
        let maze = Maze::new(data);
        verify_that!(maze.dig(23, Direction::Right), some(eq(72)))?;
        Ok(())
    }
}

fn part_1(maze: &Maze) -> usize {
    let costs = maze.costs();
    let original_dist = costs[maze.end_pos];

    (0..(maze.data.len()))
        .filter_map(|pos| maze.dig(pos, Direction::Right))
        .filter(|cost| original_dist - cost >= 100)
        .count()
        + (0..(maze.data.len()))
            .filter_map(|pos| maze.dig(pos, Direction::Down))
            .filter(|cost| original_dist - cost >= 100)
            .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let maze = Maze::new(&input);
    let costs = maze.costs();
    println!("Distance to end: {}", costs[maze.end_pos]);

    println!("Part 1: {}", part_1(&maze));
    Ok(())
}
