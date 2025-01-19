use advent_2024_rust::TileIndex;

struct Plot {
    data: Vec<char>,
    tiles: TileIndex,
}

#[derive(Debug)]
struct Region {
    name: char,
    indices: Vec<usize>,
}

impl Plot {
    fn new(s: &str) -> Plot {
        let data: Vec<char> = s.trim().lines().flat_map(str::chars).collect();
        let height = s.trim().lines().count();
        let width = data.len() / height;
        Plot {
            data,
            tiles: TileIndex { height, width },
        }
    }

    fn collect_regions(&self) -> Vec<Region> {
        let mut result = Vec::new();

        let mut visited = vec![false; self.data.len()];
        while let Some(index) = visited.iter().rposition(|x| !*x) {
            let name = self.data[index];
            let mut indices = Vec::new();
            let mut queue = vec![index];
            while let Some(neighbor) = queue.pop() {
                if visited[neighbor] {
                    continue;
                }
                visited[neighbor] = true;
                indices.push(neighbor);

                for neighbor in [
                    self.tiles.left(neighbor),
                    self.tiles.right(neighbor),
                    self.tiles.up(neighbor),
                    self.tiles.down(neighbor),
                ] {
                    queue.extend(
                        neighbor
                            .filter(|&idx| !visited[idx])
                            .filter(|&idx| self.data[idx] == name),
                    )
                }
            }

            result.push(Region { name, indices });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_collect_regions() -> Result<()> {
        let data = "
AAAA
BBCD
BBCC
EEEC
";
        let plot = Plot::new(data);
        let regions = plot.collect_regions();
        verify_that!(
            regions,
            unordered_elements_are![
                matches_pattern!(Region {
                    name: eq(&'A'),
                    indices: unordered_elements_are![&0, &1, &2, &3],
                }),
                matches_pattern!(Region {
                    name: eq(&'B'),
                    indices: unordered_elements_are![&4, &5, &8, &9],
                }),
                matches_pattern!(Region {
                    name: eq(&'C'),
                    indices: unordered_elements_are![&6, &10, &11, &15],
                }),
                matches_pattern!(Region {
                    name: eq(&'D'),
                    indices: unordered_elements_are![&7],
                }),
                matches_pattern!(Region {
                    name: eq(&'E'),
                    indices: unordered_elements_are![&12, &13, &14],
                }),
            ]
        )?;

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
