#[derive(Debug, PartialEq)]
struct Register {
    name: String,
    value: u32,
}

#[derive(Debug, PartialEq)]
struct Machine {
    a: u32,
    b: u32,
    c: u32,
    program: Box<[u8]>,
    counter: usize,
}

impl Machine {
    fn adv(&mut self) {}
    fn bxl(&mut self) {}
    fn bst(&mut self) {}
    fn jnz(&mut self) {}
    fn bxc(&mut self) {}
    fn out(&mut self) {}
    fn bdv(&mut self) {}
    fn cdv(&mut self) {}
}

mod parser {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, line_ending, u32, u8};
    use nom::multi::{separated_list0};
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

    fn parse_program(input: &str) -> IResult<&str, Box<[u8]>> {
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
                    c:0,
                    program: vec![0, 1, 5, 4, 3, 0].into(),
                    counter: 0
})))
        }
    }
}

fn main() {}
