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

    fn trailheads(&self) -> impl Iterator<Item = usize> +'_ {
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
	    results.push(index+1);
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
	if index / self.width < self.height {
	    results.push(index + self.width)
	}

	results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_trailheads() -> Result<()> {
	let field = FieldMap::new("\
0123
1234
8765
9876
	    ");
	verify_that!(field.trailheads().collect::<Vec<_>>(), [eq(&0)])
    }

    #[gtest]
    fn test_directional_neighbors() -> Result<()> {
	let field = FieldMap::new("\
0123
1234
8765
	    ");
	verify_that!(field.directional_neighbors(0), [eq(&1), eq(&4)])?;
	verify_that!(field.directional_neighbors(5), [eq(&6), eq(&4), eq(&1), eq(&9)])?;
	Ok(())
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;
    let field_map = FieldMap::new(&input);
    println!("{:?}", field_map);

    Ok(())
}
