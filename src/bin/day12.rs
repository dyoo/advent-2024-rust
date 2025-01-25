use advent_2024::TileIndex;

struct Plot<T> {
    data: Vec<T>,
    tiles: TileIndex,
}

#[derive(Debug)]
struct Region<T> {
    name: T,
    indices: Vec<usize>,
}

impl Plot<char> {
    fn new(s: &str) -> Self {
        let data: Vec<char> = s.trim().lines().flat_map(str::chars).collect();
        let height = s.trim().lines().count();
        let width = data.len() / height;
        Plot {
            data,
            tiles: TileIndex { height, width },
        }
    }
}

impl<T: PartialEq + Copy> Plot<T> {
    fn collect_regions(&self) -> Vec<Region<T>> {
        let mut result = Vec::new();

        let mut visited = vec![false; self.data.len()];
        let mut last_unvisited = visited.len();
        loop {
            let Some(index) = visited[..last_unvisited].iter().rposition(|x| !*x) else {
                break;
            };
            last_unvisited = index;

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

    fn perimeter(&self, region: &Region<T>) -> usize {
        region
            .indices
            .iter()
            .map(|&idx| {
                [
                    self.tiles.left(idx),
                    self.tiles.right(idx),
                    self.tiles.up(idx),
                    self.tiles.down(idx),
                ]
                .into_iter()
                .map(|neighbor| match neighbor {
                    None => 1,
                    Some(i) => usize::from(self.data[i] != self.data[idx]),
                })
                .sum::<usize>()
            })
            .sum()
    }

    fn sides(&self, region: &Region<T>) -> usize {
        [
            TileIndex::left,
            TileIndex::right,
            TileIndex::up,
            TileIndex::down,
        ]
        .into_iter()
        .map(|directional_indexer| {
            let edges = region.indices.iter().copied().filter(|&i| {
                let neighbor = directional_indexer(&self.tiles, i);
                match neighbor {
                    None => true,
                    Some(j) => self.data[i] != self.data[j],
                }
            });

            let mut edge_data = vec![false; self.data.len()];
            for e in edges {
                edge_data[e] = true;
            }

            (Plot {
                data: edge_data,
                tiles: self.tiles.clone(),
            })
            .collect_regions()
            .into_iter()
            .filter(|r| r.name)
            .count()
        })
        .sum()
    }
}

impl<T> Region<T> {
    fn area(&self) -> usize {
        self.indices.len()
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

    #[gtest]
    fn test_perimeters() -> Result<()> {
        let data = "
AAAA
BBCD
BBCC
EEEC
";
        let plot = Plot::new(data);
        let regions = plot.collect_regions();
        verify_that!(
            regions
                .into_iter()
                .map(|region| (region.name, plot.perimeter(&region)))
                .collect::<Vec<_>>(),
            { &('A', 10), &('B', 8), &('C', 10), &('D', 4), &('E', 8) }
        )
    }

    #[gtest]
    fn test_part_1() -> Result<()> {
        let data = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let plot = Plot::new(data);
        verify_that!(part_1(&plot), eq(1930))
    }

    #[gtest]
    fn test_sides() -> Result<()> {
        let data = "
AAAA
BBCD
BBCC
EEEC
";
        let plot = Plot::new(data);
        let regions = plot.collect_regions();
        verify_that!(
            regions
                .into_iter()
                .map(|region| (region.name, plot.sides(&region)))
                .collect::<Vec<_>>(),
            { &('A', 4), &('B', 4), &('C', 8), &('D', 4), &('E', 4) }
        )
    }

    #[gtest]
    fn test_part_2() -> Result<()> {
        let data = "
AAAA
BBCD
BBCC
EEEC
";
        let plot = Plot::new(data);
        verify_that!(part_2(&plot), eq(80))
    }

    #[gtest]
    fn test_part_2_intermediate() -> Result<()> {
        let data = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let plot = Plot::new(data);
        verify_that!(part_2(&plot), eq(368))
    }
}

fn part_1(plot: &Plot<char>) -> usize {
    let regions = plot.collect_regions();
    regions
        .into_iter()
        .map(|region| region.area() * plot.perimeter(&region))
        .sum()
}

fn part_2(plot: &Plot<char>) -> usize {
    let regions = plot.collect_regions();
    regions
        .into_iter()
        .map(|region| region.area() * plot.sides(&region))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::io::read_to_string(std::io::stdin())?;
    let plot = Plot::new(&data);
    println!("Part 1: {}", part_1(&plot));
    println!("Part 2: {}", part_2(&plot));
    Ok(())
}
