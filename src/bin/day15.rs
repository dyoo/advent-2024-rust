use advent_2024::TileIndex;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Sokoban {
    data: Vec<char>,
    tiles: TileIndex,
}

impl Sokoban {
    fn push(&mut self, at: usize, dir: Direction) {}
}

impl FromStr for Sokoban {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let s = s.trim();
	let data: Vec<char> = s.lines().flat_map(|line| line.trim().chars()).collect();
	let height = s.lines().count();
	let width = data.len() / height;
	Ok(Self {data, tiles: TileIndex {width, height}})
    }
}

fn parse_map(s: &str) {}
fn parse_directions(s: &str) {}

fn main() {}
