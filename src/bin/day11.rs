use std::collections::HashMap;

fn parse(s: &str) -> impl Iterator<Item = Result<u64, std::num::ParseIntError>> + '_ {
    s.split_whitespace().map(|s| s.parse::<u64>())
}

fn blink_all(numbers: impl IntoIterator<Item = u64>) -> Vec<u64> {
    numbers.into_iter().flat_map(blink).collect()
}

#[test]
fn test_blink_all() {
    assert_eq!(blink_all([125, 17]), vec![253000, 1, 7]);
    assert_eq!(blink_all([253000, 1, 7]), vec![253, 0, 2024, 14168]);
}

fn blink(number: u64) -> Vec<u64> {
    match number {
        0 => vec![1],
        n if even_digits(n) => split_halves(n),
        _ => vec![number.checked_mul(2024).expect("overflow")],
    }
}

fn even_digits(number: u64) -> bool {
    (number.ilog10() + 1) % 2 == 0
}

fn split_halves(number: u64) -> Vec<u64> {
    let modulo = 10u64.pow((number.ilog10() + 1) / 2);
    vec![number / modulo, number % modulo]
}

fn histogram(numbers: impl IntoIterator<Item = u64>) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    for n in numbers {
        *map.entry(n).or_default() += 1;
    }
    map
}

fn blink_all_counting(numbers: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    for (n, factor) in numbers {
        for v in blink(n) {
            *map.entry(v).or_default() += factor;
        }
    }
    map
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::read_to_string(std::io::stdin())?;

    {
        let mut values: Vec<u64> = parse(&input).collect::<Result<Vec<_>, _>>()?;
        for _ in 0..25 {
            values = blink_all(values);
        }
        println!("Part 1: {:?}", values.len());

        let mut values_map: HashMap<u64, u64> =
            histogram(parse(&input).collect::<Result<Vec<_>, _>>()?);
        for _ in 0..25 {
            values_map = blink_all_counting(values_map);
        }
        println!("Part 1: {:?}", values_map.values().sum::<u64>());

        let mut values_map: HashMap<u64, u64> =
            histogram(parse(&input).collect::<Result<Vec<_>, _>>()?);
        for _ in 0..75 {
            values_map = blink_all_counting(values_map);
        }
        println!("Part 2: {:?}", values_map.values().sum::<u64>());
    }

    Ok(())
}
