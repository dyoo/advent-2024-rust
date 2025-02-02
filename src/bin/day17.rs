#![allow(dead_code, unused_variables)]

type Integer = u32;
type Opcode = u8;

#[derive(Debug, PartialEq)]
struct Register {
    name: String,
    value: Integer,
}

#[derive(Debug, PartialEq)]
struct Machine {
    a: Integer,
    b: Integer,
    c: Integer,
    program: Box<[Opcode]>,
    counter: usize,
    out: Vec<Integer>,
}


impl Machine {
    fn run(&mut self) {
        while self.counter < self.program.len() - 1 {
            let decoded = self.decode_next_instruction(self.program[self.counter]);
            let operand = self.program[self.counter + 1];
            decoded(self, operand);
        }
    }

    fn decode_next_instruction(&mut self, opcode: Opcode) -> fn(&mut Machine, Opcode) {
        match opcode {
            0 => Machine::adv,
            1 => Machine::bxl,
            2 => Machine::bst,
            3 => Machine::jnz,
            4 => Machine::bxc,
            5 => Machine::out,
            6 => Machine::bdv,
            7 => Machine::cdv,
            _ => panic!("Unexpected fall through"),
        }
    }

    fn literal_operand(&self, operand: Opcode) -> Integer {
        operand as Integer
    }

    fn combo_operand(&self, operand: Opcode) -> Integer {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unexpected fallthrough"),
        }
    }

    fn adv(&mut self, operand: Opcode) {
        let numerator: Integer = self.a;
        let denominator: Integer = (2 as Integer).pow(self.combo_operand(operand));
        self.a = numerator / denominator;
        self.counter += 2;
    }
    
    fn bdv(&mut self, operand: Opcode) {
        let numerator: Integer = self.a;
        let denominator: Integer = (2 as Integer).pow(self.combo_operand(operand));
        self.b = numerator / denominator;
        self.counter += 2;
    }

    fn cdv(&mut self, operand: Opcode) {
        let numerator: Integer = self.a;
        let denominator: Integer = (2 as Integer).pow(self.combo_operand(operand));
        self.c = numerator / denominator;
        self.counter += 2;
    }
    
    fn bxl(&mut self, operand: Opcode) {
        self.b = self.b ^ self.literal_operand(operand);
        self.counter += 2;
    }

    fn bst(&mut self, operand: Opcode) {
        self.b = self.combo_operand(operand) % 8;
        self.counter += 2;
    }

    fn jnz(&mut self, operand: Opcode) {
        if self.a == 0 {
            self.counter += 2;
        } else {
            let v = self.literal_operand(operand);
            self.counter = v as usize;
        }
    }

    fn bxc(&mut self, operand: Opcode) {
        self.b = self.b ^ self.c;
        self.counter += 2;
    }
    
    fn out(&mut self, operand: Opcode) {
        let output = self.combo_operand(operand) % 8;
        self.out.push(output);
        self.counter += 2;
    }
}

mod parser {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, line_ending, u32, u8};
    use nom::multi::separated_list0;
    use nom::sequence::terminated;
    use nom::IResult;

    fn parse_register(input: &str) -> IResult<&str, Register> {
        let (input, _) = tag("Register ")(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, value) = u32(input)?;
        Ok((
            input,
            Register {
                name: name.to_string(),
                value,
            },
        ))
    }

    fn parse_program(input: &str) -> IResult<&str, Box<[Opcode]>> {
        let (input, _) = tag("Program: ")(input)?;
        let (input, vs) = separated_list0(tag(","), u8)(input)?;
        Ok((input, vs.into()))
    }

    fn parse_machine(input: &str) -> IResult<&str, Machine> {
        let (input, register_a) = terminated(parse_register, line_ending)(input)?;
        // TODO: see how to build our own parse error if this isn't named "A".
        let (input, register_b) = terminated(parse_register, line_ending)(input)?;
        let (input, register_c) = terminated(parse_register, line_ending)(input)?;

        let (input, _) = line_ending(input)?;
        let (input, instructions) = parse_program(input)?;
        Ok((
            input,
            Machine {
                a: register_a.value,
                b: register_b.value,
                c: register_c.value,
                program: instructions.into(),
                counter: 0,
                out: Vec::new(),
            },
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use googletest::prelude::*;

        #[gtest]
        fn test_parse_register() -> Result<()> {
            let (input, r) = parse_register("Register A: 729")?;
            verify_that!(input, eq(""))?;
            verify_that!(
                r,
                eq(&Register {
                    name: "A".to_string(),
                    value: 729
                })
            )?;
            Ok(())
        }

        #[gtest]
        fn test_parse_program() -> Result<()> {
            let (input, p) = parse_program("Program: 0,1,5,4,3,0")?;
            verify_that!(input, eq(""))?;
            verify_that!(p, eq(&vec![0, 1, 5, 4, 3, 0].into()))?;
            Ok(())
        }

        #[gtest]
        fn test_parse_machine() -> Result<()> {
            let data = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
            let (input, machine) = parse_machine(data)?;
            verify_that!(input, eq("\n"))?;
            verify_that!(
                machine,
                eq(&(Machine {
                    a: 729,
                    b: 0,
                    c: 0,
                    program: vec![0, 1, 5, 4, 3, 0].into(),
                    counter: 0,
                    out: Vec::new(),
                }))
            )
        }

        #[gtest]
        fn test_ex1() -> Result<()> {
            let mut machine = Machine {
                a: 0,
                b: 0,
                c: 9,
                program: [2, 6].into(),
                counter: 0,
                out: Vec::new(),
            };
            machine.run();
            verify_that!(machine.b, eq(1))
        }

        #[gtest]
        fn test_ex2() -> Result<()> {
            let mut machine = Machine {
                a: 10,
                b: 0,
                c: 0,
                program: [5, 0, 5, 1, 5, 4].into(),
                counter: 0,
                out: Vec::new(),
            };
            machine.run();
            verify_that!(machine.out, [&0, &1, &2])
        }

        #[gtest]
        fn test_ex3() -> Result<()> {
            let mut machine = Machine {
                a: 2024,
                b: 0,
                c: 0,
                program: [0, 1, 5, 4, 3, 0].into(),
                counter: 0,
                out: Vec::new(),
            };
            machine.run();
            verify_that!(machine.out, [&4, &2, &5, &6, &7, &7, &7, &7, &3, &1, &0])?;
            verify_that!(machine.a, eq(0))?;
            Ok(())
        }        
    }
}

fn main() {}
