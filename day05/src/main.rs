use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
struct Problem {
    orderings: Vec<(u32, u32)>,
    numbers: Vec<Vec<u32>>,
}

fn parse(s: impl AsRef<str>) -> Result<Problem, Box<dyn Error>> {
    let mut sections = s.as_ref().split("\n\n");
    let orderings = sections
        .next()
        .ok_or("Missing ordering section")?
        .lines()
        .map(|line| {
            line.split("|")
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| Box::<dyn Error>::from(err))
                .and_then(|numbers| {
                    numbers
                        .get(0)
                        .ok_or(Box::<dyn Error>::from(format!(
                            "lhs missing from {:?}",
                            line
                        )))
                        .and_then(|&n1| {
                            numbers
                                .get(1)
                                .ok_or(Box::<dyn Error>::from(format!(
                                    "rhs missing from {:?}",
                                    line
                                )))
                                .map(|&n2| (n1, n2))
                        })
                })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let numbers = sections
        .next()
        .ok_or("Missing numbers section")?
        .lines()
        .map(|line| {
            line.split(",")
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| Box::<dyn Error>::from(err))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(Problem { orderings, numbers })
}

fn filter_correct_orderings(p: &Problem) -> Vec<&Vec<u32>> {
    p.numbers
        .iter()
        .filter(|numbers| {
            // Record the child->parent dependencies for quicker lookup.
            let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
            {
                let seen_in_numbers: HashSet<u32> = numbers.iter().copied().collect();
                for (parent, child) in p.orderings.iter() {
                    if seen_in_numbers.contains(&parent) && seen_in_numbers.contains(&child) {
                        rules.entry(*child).or_default().insert(*parent);
                    }
                }
            }

            let mut seen: HashSet<u32> = HashSet::new();

            for &n in numbers.iter() {
                if rules
                    .entry(n)
                    .or_default()
                    .iter()
                    .any(|parent| !seen.contains(parent))
                {
                    return false;
                }

                seen.insert(n);
            }

            true
        })
        .collect()
}

struct TopologicalSort<T> {
    available: Vec<T>,
    pending: HashMap<T, Vec<T>>,
    counts: HashMap<T, usize>,
}

impl<T> TopologicalSort<T>
where
    T: Eq + Hash + Copy,
{
    fn new(deps: impl IntoIterator<Item = (T, T)>) -> Self {
        let mut pending: HashMap<T, Vec<T>> = HashMap::new();
        let mut counts: HashMap<T, usize> = HashMap::new();

        let mut seen = HashSet::new();
        for (parent, child) in deps {
            seen.insert(parent);
            seen.insert(child);

            pending.entry(parent).or_default().push(child);
            *counts.entry(child).or_insert(0) += 1;
        }
        for child in counts.keys() {
            seen.remove(child);
        }
        let available: Vec<T> = seen.into_iter().collect();

        Self {
            available,
            pending,
            counts,
        }
    }
}

impl<T> Iterator for TopologicalSort<T>
where
    T: Eq + Hash + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.available.pop();

        if let Some(parent) = result {
            let children = self
                .pending
                .remove(&parent)
                .into_iter()
                .flat_map(|children| children);
            for child in children {
                let child_count = self.counts.entry(child).or_insert(1);
                *child_count = child_count.saturating_sub(1);
                if *child_count == 0 {
                    self.available.push(child);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const TEST_DATA: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[gtest]
    fn test_parse() -> Result<()> {
        let problem = parse(
            "\
	    45|53
97|13

75,47,61
97,61,53",
        );
        verify_that!(
            problem,
            ok(eq(&Problem {
                orderings: vec![(45, 53), (97, 13)],
                numbers: vec![vec![75, 47, 61], vec![97, 61, 53],]
            }))
        )
    }

    #[gtest]
    fn test_filtering() -> Result<()> {
        let problem = parse(TEST_DATA).unwrap();
        verify_that!(
            filter_correct_orderings(&problem),
            container_eq(vec![
                &vec![75, 47, 61, 53, 29],
                &vec![97, 61, 53, 29, 13],
                &vec![75, 29, 13],
            ])
        )
    }

    #[gtest]
    fn test_topological() -> Result<()> {
        let mut topsort = TopologicalSort::new([(2, 3), (1, 2)]);
        verify_that!(topsort.next(), some(eq(1)))?;
        verify_that!(topsort.next(), some(eq(2)))?;
        verify_that!(topsort.next(), some(eq(3)))?;
        verify_that!(topsort.next(), none())?;
        Ok(())
    }

    #[gtest]
    fn test_topological2() -> Result<()> {
        // This tests the last example given in part 2's description,
        // since it's the most sophisticated.
        let mut topsort = TopologicalSort::new([
            (29, 13),
            (47, 13),
            (75, 13),
            (97, 13),
            (75, 47),
            (97, 47),
            (97, 75),
            (47, 29),
            (75, 29),
            (97, 29),
        ]);
        verify_that!(topsort.next(), some(eq(97)))?;
        verify_that!(topsort.next(), some(eq(75)))?;
        verify_that!(topsort.next(), some(eq(47)))?;
        verify_that!(topsort.next(), some(eq(29)))?;
        verify_that!(topsort.next(), some(eq(13)))?;
        verify_that!(topsort.next(), none())?;
        Ok(())
    }
}

fn middle(v: &[u32]) -> u32 {
    v[v.len() / 2]
}

fn part1(p: &Problem) -> u32 {
    filter_correct_orderings(p)
        .into_iter()
        .map(|ordering| middle(ordering))
        .sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = parse(std::io::read_to_string(std::io::stdin())?)?;
    println!("Part 1: {}", part1(&problem));
    Ok(())
}
