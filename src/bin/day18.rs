use advent_2024::{Direction, TileIndex};

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, u8};
    use nom::multi::many0;
    use nom::sequence::separated_pair;
    use nom::sequence::terminated;
    use nom::IResult;

    pub fn parse_coord(input: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, tag(","), u8)(input)
    }

    pub fn parse_coords(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
        many0(terminated(parse_coord, line_ending))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use googletest::prelude::*;

        #[gtest]
        fn test_parse_pair() -> Result<()> {
            let (input, coord) = parse_coord("10,27")?;
            verify_that!(input, eq(""))?;
            verify_that!(coord, eq((10, 27)))?;

            Ok(())
        }

        #[gtest]
        fn test_parse_coords() -> Result<()> {
            let (input, coord) = parse_coords("1,2\n3,4\n")?;
            verify_that!(input, eq(""))?;
            verify_that!(coord, [&(1, 2), &(3, 4)])?;

            Ok(())
        }
    }
}

struct Grid {
    data: Vec<bool>,
    tile_index: TileIndex,
}

impl Grid {
    fn new(width: u8, height: u8) -> Self {
        Self {
            data: vec![false; width as usize * height as usize],
            tile_index: TileIndex {
                width: width as usize,
                height: height as usize,
            },
        }
    }
    fn height(&self) -> usize {
        self.tile_index.height
    }

    fn mark(&mut self, coord: (u8, u8)) {
        let index = self.height() * coord.1 as usize + coord.0 as usize;
        self.data[index] = true;
    }

    fn step_count(&self) -> Option<u32> {
        let mut visited = vec![false; self.data.len()];
        let mut to_visit = vec![0];
        let mut count = 0;
        while !to_visit.is_empty() {
            let mut to_visit_next = Vec::new();

            for index in to_visit {
                if visited[index] {
                    continue;
                }
                visited[index] = true;
                if index == visited.len() -1  {
                    return Some(count);
                }
                
                for dir in [
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                ] {
                    to_visit_next.extend(
                        self.tile_index
                            .dir_to(index, dir)
                            .filter(|idx| !visited[*idx] && !self.data[*idx]),
                    );
                }
            }
            count += 1;            
            to_visit = to_visit_next;
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_step_count() -> Result<()> {
        let data = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
";
        let (_, coords): (_, Vec<(u8, u8)>) =
            parser::parse_coords(data).map_err(|e| e.to_owned())?;
        let mut grid = Grid::new(7, 7);
        for c in coords {
            grid.mark(c);
        }
        verify_that!(grid.step_count(), some(eq(22)))
    }
}

fn my_binary_search(n: usize, pred: impl Fn(usize) -> bool) -> usize {
    let mut start = 0;
    let mut end = n;
    // Loop invariant: some element in my_range makes the predicate false.
    while start < end {
        let mid = start + (end - start) / 2;
        if pred(mid) {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    start
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let (_, coords): (_, Vec<(u8, u8)>) = parser::parse_coords(&input).map_err(|e| e.to_owned())?;
    let mut grid = Grid::new(71, 71);
    for c in &coords[..1024] {
        grid.mark(*c);
    }
    println!("Part 1: {:?}", grid.step_count());
    
    let mut grid = Grid::new(71, 71);
    for c in &coords {
        grid.mark(*c);
        if grid.step_count().is_none() {
            println!("Part 2: {:?}", *c);
            break;
        }
    }

    // Other folks suggested using binary search, so let's try that approach too.
    let idx = my_binary_search(coords.len(),
        |n| {
            let mut grid = Grid::new(71, 71);
            for c in &coords[..=n] {
                grid.mark(*c);
            }
            grid.step_count().is_some()
        });
    println!("idx: {:?}, coord: {:?}", idx, coords[idx]);
    
    Ok(())
}
