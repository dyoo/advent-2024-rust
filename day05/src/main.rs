use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, PartialEq)]
struct Problem {
    orderings: Vec<PageOrdering>,
    numbers: Vec<PageNumbers>,
}

#[derive(Debug, PartialEq)]
struct PageOrdering(u32, u32);

#[derive(Debug, PartialEq)]
struct PageNumbers(Vec<u32>);

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
                                .map(|&n2| PageOrdering(n1, n2))
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
                .map(|numbers| PageNumbers(numbers))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(Problem { orderings, numbers })
}

fn filter_correct_orderings(p: &Problem) -> Vec<&PageNumbers> {
    // Record the child->parent dependencies for quicker lookup.
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    {
        let mut seen: HashSet<u32> = p.numbers.iter().flat_map(|n| n.0.iter()).copied().collect();
        for PageOrdering(parent, child) in p.orderings.iter() {
            if seen.contains(&parent) && seen.contains(&child) {
                rules.entry(*child).or_default().insert(*parent);
            }
        }
    }

    p.numbers
        .iter()
        .filter(|numbers| {
            let mut seen: HashSet<u32> = HashSet::new();

            for &n in numbers.0.iter() {
                if rules
                    .entry(n)
                    .or_default()
                    .iter()
                    .any(|p| !seen.contains(p))
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
                orderings: vec![PageOrdering(45, 53), PageOrdering(97, 13)],
                numbers: vec![
                    PageNumbers(vec![75, 47, 61],),
                    PageNumbers(vec![97, 61, 53])
                ]
            }))
        )
    }
}

fn main() {
    println!("Hello, world!");
}
