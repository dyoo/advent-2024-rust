mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, u8};
    use nom::multi::many0;
    use nom::sequence::separated_pair;
    use nom::sequence::terminated;
    use nom::IResult;

    pub fn parse_coord(input: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(u8, tag(","), u8)(input)
    }

    pub fn parse_coords(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
        many0(terminated(parse_coord, line_ending))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use googletest::prelude::*;

        #[gtest]
        fn test_parse_pair() -> Result<()> {
            let (input, coord) = parse_coord("10,27")?;
            verify_that!(input, eq(""))?;
            verify_that!(coord, eq((10, 27)))?;

            Ok(())
        }

        #[gtest]
        fn test_parse_coords() -> Result<()> {
            let (input, coord) = parse_coords("1,2\n3,4\n")?;
            verify_that!(input, eq(""))?;
            verify_that!(coord, [&(1, 2), &(3, 4)])?;

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = std::io::read_to_string(std::io::stdin())?;
    let coords = parser::parse_coords(&input).map_err(|e| e.to_owned())?;
    dbg!(&coords);
    Ok(())
}
 
