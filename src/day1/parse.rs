use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::tuple,
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub turn: Turn,
    pub distance: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Turn {
    Right,
    Left,
}

impl Turn {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        alt((value(Turn::Left, tag("L")), value(Turn::Right, tag("R"))))(i)
    }
}

impl Instruction {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((Turn::parse, nom::character::complete::u32)),
            |(turn, distance)| Self { turn, distance },
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(
            Instruction::parse("R1").unwrap().1,
            Instruction {
                turn: Turn::Right,
                distance: 1
            }
        );

        assert!(Instruction::parse("R-1").is_err());
        assert!(Instruction::parse("P2").is_err());
        assert!(Instruction::parse("").is_err());
    }
}
