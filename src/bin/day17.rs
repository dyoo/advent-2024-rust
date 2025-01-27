#[derive(Debug, PartialEq)]
struct Register {
    name: String,
    value: u32,
}

mod parser {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, line_ending, u32, u8};
    use nom::multi::{many1, separated_list0};
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

    fn parse_input(input: &str) -> IResult<&str, (Box<[Register]>, Box<[u8]>)> {
        let (input, registers) = many1(terminated(parse_register, line_ending))(input)?;
        let (input, _) = line_ending(input)?;
        let (input, instructions) = parse_program(input)?;
        Ok((input, (registers.into(), instructions)))
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
        fn test_parse_input() -> Result<()> {
            let data = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
            let (input, result) = parse_input(data)?;
            verify_that!(input, eq("\n"))?;
            verify_that!(
                result,
                eq(&(
                    vec![
                        Register {
                            name: "A".to_string(),
                            value: 729
                        },
                        Register {
                            name: "B".to_string(),
                            value: 0
                        },
                        Register {
                            name: "C".to_string(),
                            value: 0
                        },
                    ]
                    .into(),
                    vec![0, 1, 5, 4, 3, 0].into()
                ))
            )
        }
    }
}

fn main() {}
