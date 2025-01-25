use regex::Regex;

#[derive(Debug)]
struct Parser {
    pattern: Regex,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul { lhs: i32, rhs: i32 },
}

impl Instruction {
    fn is_mul(&self) -> bool {
        matches!(self, Instruction::Mul { .. })
    }
}

#[derive(Debug, PartialEq)]
struct State {
    enabled: bool,
    val: i32,
}

impl State {
    fn new() -> Self {
        State {
            enabled: true,
            val: 0,
        }
    }
    fn eval(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Do => {
                self.enabled = true;
            }
            Instruction::Dont => {
                self.enabled = false;
            }
            Instruction::Mul { lhs, rhs } => {
                if self.enabled {
                    self.val += lhs * rhs;
                }
            }
        }
    }
}

impl Parser {
    fn new() -> Self {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        Parser { pattern }
    }

    fn parse(&self, s: &str) -> Vec<Instruction> {
        let mut result = Vec::new();
        for captures in self.pattern.captures_iter(s) {
            match &captures[0] {
                "do()" => {
                    result.push(Instruction::Do);
                }
                "don't()" => {
                    result.push(Instruction::Dont);
                }
                _ => {
                    let lhs = &captures[1];
                    let rhs = &captures[2];

                    let Some(lhs): Option<i32> = lhs.parse().ok() else {
                        continue;
                    };
                    let Some(rhs): Option<i32> = rhs.parse().ok() else {
                        continue;
                    };
                    result.push(Instruction::Mul { lhs, rhs });
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

    #[gtest]
    fn test_parse() -> Result<()> {
        let parser = Parser::new();
        let instructions =
            parser.parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        verify_that!(
            instructions,
            eq(&vec![
                Instruction::Mul { lhs: 2, rhs: 4 },
                Instruction::Mul { lhs: 5, rhs: 5 },
                Instruction::Mul { lhs: 11, rhs: 8 },
                Instruction::Mul { lhs: 8, rhs: 5 }
            ])
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let body = std::io::read_to_string(std::io::stdin())?;
    let instructions = parser.parse(&body);

    {
        let mut state = State::new();
        for instruction in instructions.iter().filter(|&x| Instruction::is_mul(x)) {
            state.eval(instruction);
        }
        println!("Part 1: {:?}", state);
    }

    {
        let mut state = State::new();
        for instruction in instructions.iter() {
            state.eval(instruction)
        }
        println!("Part 2: {:?}", state);
    }

    Ok(())
}
