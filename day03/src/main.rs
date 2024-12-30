use regex::Regex;

#[derive(Debug)]
struct Parser {
    pattern: Regex,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    lhs: i32,
    rhs: i32,
}

impl Instruction {
    fn eval(&self) -> i32 {
        self.lhs * self.rhs
    }
}

impl Parser {
    fn new() -> Self {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        Parser { pattern }
    }

    fn parse(&self, s: &str) -> Vec<Instruction> {
        let items = self.pattern.captures_iter(s).map(|capture| {
            (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            )
        });
        items
            .filter_map(|(lhs, rhs)| {
                lhs.parse::<i32>()
                    .and_then(|lhs| rhs.parse::<i32>().map(|rhs| Instruction { lhs, rhs }))
                    .ok()
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_parse() -> Result<()> {
        let parser = Parser::new();
        let instructions =
            parser.parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        verify_that!(
            instructions,
            eq(&vec![
                Instruction { lhs: 2, rhs: 4 },
                Instruction { lhs: 5, rhs: 5 },
                Instruction { lhs: 11, rhs: 8 },
                Instruction { lhs: 8, rhs: 5 }
            ])
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let body = std::io::read_to_string(std::io::stdin())?;
    let instructions = parser.parse(&body);

    for instruction in &instructions {
        println!("{:?}, {}", instruction, instruction.eval());
    }
    println!(
        "{}",
        instructions.iter().map(Instruction::eval).sum::<i32>()
    );

    Ok(())
}
