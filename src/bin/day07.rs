use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct Equation {
    test_value: u64,
    args: Box<[u64]>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        is_valid(self.test_value, self.args.as_ref())
    }

    fn is_valid2(&self) -> bool {
        is_valid2(self.test_value, self.args.as_ref())
    }
}

fn is_valid(test_val: u64, args: &[u64]) -> bool {
    if args.len() == 0 {
        return false;
    } else if args.len() == 1 {
        return test_val == args[0];
    }

    let last = *args.last().unwrap();

    if test_val >= last && is_valid(test_val - last, &args[0..args.len() - 1]) {
        return true;
    }

    if test_val % last == 0 && is_valid(test_val / last, &args[0..args.len() - 1]) {
        return true;
    }

    false
}

fn is_valid2(test_val: u64, args: &[u64]) -> bool {
    if args.len() == 0 {
        return false;
    } else if args.len() == 1 {
        return test_val == args[0];
    }

    let last = *args.last().unwrap();

    if test_val >= last && is_valid2(test_val - last, &args[0..args.len() - 1]) {
        return true;
    }

    if test_val % last == 0 && is_valid2(test_val / last, &args[0..args.len() - 1]) {
        return true;
    }

    if let Some(v) = try_unconcat(test_val, last) {
        is_valid2(v, &args[0..args.len() - 1])
    } else {
        false
    }
}

impl std::str::FromStr for Equation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let mut vals = s
            .split([' ', ':'])
            .filter_map(|v| str::parse::<u64>(v).ok());

        let test_value: u64 = vals.next().ok_or_else(|| "No test value".to_string())?;
        let args: Vec<u64> = vals.collect();

        Ok(Equation {
            test_value,
            args: args.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const DATA: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[gtest]
    fn test_parse() -> Result<()> {
        let mut parsed = DATA.lines().map(str::parse::<Equation>);
        verify_that!(
            parsed.next(),
            some(ok(eq(&Equation {
                test_value: 190,
                args: vec![10, 19].into()
            })))
        )?;
        verify_that!(
            parsed.next(),
            some(ok(eq(&Equation {
                test_value: 3267,
                args: vec![81, 40, 27].into()
            })))
        )?;

        // Check parsing of last line:
        verify_that!(
            parsed.last(),
            some(ok(eq(&Equation {
                test_value: 292,
                args: vec![11, 6, 16, 20].into()
            })))
        )?;

        Ok(())
    }

    #[gtest]
    fn test_is_valid() -> Result<()> {
        verify_that!(
            "190: 10 19".parse::<Equation>().unwrap().is_valid(),
            is_true()
        )?;
        verify_that!(
            "3267: 81 40 27".parse::<Equation>().unwrap().is_valid(),
            is_true()
        )?;

        Ok(())
    }

    #[gtest]
    fn test_part_1() -> Result<()> {
        let mut problem = DATA
            .lines()
            .map(str::parse::<Equation>)
            .collect::<std::result::Result<Vec<Equation>, _>>()
            .unwrap();

        verify_that!(part_1(&problem), eq(3749))
    }

    #[gtest]
    fn test_count_digits() -> Result<()> {
        verify_that!(count_digits(1), eq(1))?;
        verify_that!(count_digits(5), eq(1))?;
        verify_that!(count_digits(9), eq(1))?;
        verify_that!(count_digits(10), eq(2))?;
        verify_that!(count_digits(99), eq(2))?;
        verify_that!(count_digits(100), eq(3))?;
        Ok(())
    }

    #[gtest]
    fn test_try_unconcat() -> Result<()> {
        verify_that!(try_unconcat(156, 6), some(eq(15)))?;
        verify_that!(try_unconcat(178, 78), some(eq(1)))?;
        Ok(())
    }

    #[gtest]
    fn test_part_2() -> Result<()> {
        let mut problem = DATA
            .lines()
            .map(str::parse::<Equation>)
            .collect::<std::result::Result<Vec<Equation>, _>>()
            .unwrap();

        verify_that!(part_2(&problem), eq(11387))
    }
}

fn count_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn try_unconcat(n: u64, suffix: u64) -> Option<u64> {
    if n < suffix {
        None
    } else {
        let modulo = 10u64.pow(count_digits(suffix));
        if n % modulo == suffix {
            Some(n / modulo)
        } else {
            None
        }
    }
}

fn part_1(problem: &[Equation]) -> u64 {
    problem
        .par_iter()
        .filter(|e| e.is_valid())
        .map(|e| u64::from(e.test_value))
        .sum()
}

fn part_2(problem: &[Equation]) -> u64 {
    problem
        .par_iter()
        .filter(|e| e.is_valid2())
        .map(|e| u64::from(e.test_value))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem: Vec<Equation> = std::io::read_to_string(std::io::stdin())?
        .lines()
        .map(str::parse::<Equation>)
        .collect::<Result<Vec<_>, _>>()?;
    println!("Part 1: {}", part_1(&problem));
    println!("Part 2: {}", part_2(&problem));

    Ok(())
}
