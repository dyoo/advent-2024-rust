#[derive(Debug, PartialEq)]
struct Pos(i32, i32);

#[derive(Debug, PartialEq)]
struct Vel(i32, i32);

mod parser {
    use super::*;

    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, line_ending, space1};
    use nom::multi::{many1, separated_list0};
    use nom::IResult;

    pub fn parse_position(input: &str) -> IResult<&str, Pos> {
        let (input, _) = tag("p=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i32(input)?;
        Ok((input, Pos(x,y)))
    }

    pub fn parse_velocity(input: &str) -> IResult<&str, Vel> {
        let (input, _) = tag("v=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i32(input)?;
        Ok((input, Vel(x,y)))
    }

    pub fn parse_line(input: &str) -> IResult<&str, (Pos, Vel)> {
        let (input, pos) = parse_position(input)?;
        let (input, _) = space1(input)?;
        let (input, vel) = parse_velocity(input)?;
        Ok((input, (pos, vel)))
    }

    pub fn parse_all_lines(input: &str) -> IResult<&str, Vec<(Pos, Vel)>> {
        separated_list0(many1(line_ending), parse_line)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use googletest::prelude::*;

        #[gtest]
        fn test_parse_line() -> Result<()> {
            verify_that!(parse_line("p=0,4 v=3,-3")?,
                         eq(&("", (Pos(0, 4), Vel(3, -3)))))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let (_, results) = parser::parse_all_lines(&std::io::read_to_string(std::io::stdin())?).map_err(|e|e.to_owned())?;
    for r in results {
        println!("{:?}", r);
    }

    Ok(())
}
