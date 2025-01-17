#[derive(Debug, PartialEq)]
struct FieldMap {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl FieldMap {
    fn new(s: &str) -> Self {
        let data: Vec<_> = s
            .trim()
            .lines()
            .flat_map(|l| l.chars().map(|ch| ch as u8 - '0' as u8))
            .collect();
        let height = s.trim().lines().count();
        let width = data.len() / height;

        Self {
            data,
            height,
            width,
        }
    }

    fn trailheads(&self) -> impl Iterator<Item = usize> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &height)| height == 0)
            .map(|(index, _)| index)
    }

    fn directional_neighbors(&self, index: usize) -> Vec<usize> {
        let mut results = Vec::new();
        // right
        if index % self.width + 1 < self.width && index + 1 < self.data.len() {
            results.push(index + 1);
        }

        // left
        if index % self.width > 0 && index != 0 {
            results.push(index - 1)
        }

        // up
        if index / self.width > 0 {
            results.push(index - self.width)
        }

        // down
        if index / self.width < self.height - 1 {
            results.push(index + self.width)
        }

        results
    }

    fn neighbors(&self, i: usize) -> impl Iterator<Item=usize> + '_ {
	self.directional_neighbors(i).into_iter().filter(move |j| self.data[i] + 1 == self.data[*j])
    }

    fn dfs(&self, start: impl IntoIterator<Item=usize>) -> Vec<usize> {
	let mut to_visit: Vec<_> = start.into_iter().collect();
	let mut visited = vec![false; self.data.len()];
	while let Some(index) = to_visit.pop() {
	    if visited[index] {
		continue;
	    }
	    visited[index] = true;
	    to_visit.extend(self.neighbors(index));
	}
	visited.into_iter().enumerate().filter(|(_, v)| *v).map(|(index, _)| index).collect()
    }

    fn trailhead_score(&self, trailhead:usize) -> usize {
	let visited = self.dfs([trailhead]);
	visited.into_iter().filter(|index| self.data[*index] == 9).count()
    }
}


fn part_1(field_map: &FieldMap)-> usize {
    field_map.trailheads().map(|trailhead| field_map.trailhead_score(trailhead)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_trailheads() -> Result<()> {
        let field = FieldMap::new(
            "\
0123
1234
8765
9876
	    ",
        );
        verify_that!(field.trailheads().collect::<Vec<_>>(), [eq(&0)])
    }

    #[gtest]
    fn test_directional_neighbors() -> Result<()> {
        let field = FieldMap::new(
            "\
0123
1234
8765
	    ",
        );
        verify_that!(
            field.directional_neighbors(0),
            unordered_elements_are![eq(&1), eq(&4)]
        )?;
        verify_that!(
            field.directional_neighbors(5),
            unordered_elements_are![eq(&6), eq(&4), eq(&1), eq(&9)]
        )?;
        verify_that!(
            field.directional_neighbors(11),
            unordered_elements_are![eq(&10), eq(&7)]
        )?;
	verify_that!(
	    field.directional_neighbors(10),
            unordered_elements_are![eq(&9), eq(&11), eq(&6)]
        )?;

        Ok(())
    }

    #[gtest]
    fn test_neighbors() -> Result<()> {
        let field = FieldMap::new(
            "\
0123
1234
8765
	    ",
        );
        verify_that!(
            field.neighbors(1).collect::<Vec<_>>(),
            unordered_elements_are![eq(&2), eq(&5)]
        )?;

        Ok(())
    }

    #[gtest]
    fn test_dfs() -> Result<()> {
        let field = FieldMap::new(
            "\
0023
1234
8765
	    ",
        );
        verify_that!(
            field.dfs([0]),
            unordered_elements_are![eq(&0), eq(&4), eq(&5), eq(&6), eq(&7),
	    eq(&8), eq(&9), eq(&10), eq(&11)]
        )?;

        Ok(())
    }


    #[gtest]
    fn test_trailhead_score() -> Result<()> {
        let field = FieldMap::new(
            "\
0123
1234
8765
9876
	    ",
        );
	verify_that!(field.trailhead_score(0),
		     eq(1))?;
	Ok(())
    }

    #[gtest]
    fn test_part_1() -> Result<()> {
	let data = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
	let field = FieldMap::new(data);
	verify_that!(part_1(&field),
		     eq(36))
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let field_map = FieldMap::new(&input);
    println!("Part 1: {:?}", part_1(&field_map));

    Ok(())
}
