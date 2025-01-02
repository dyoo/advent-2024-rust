use std::collections::{HashMap, HashSet};
use std::error::Error;

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
