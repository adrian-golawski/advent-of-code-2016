use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, u8},
    combinator::map,
    sequence::tuple,
    IResult,
};

pub fn solve(input: &str) {
    let p = input
        .split(", ")
        .map(Instruction::from)
        .map(Result::unwrap)
        .fold(Pointer::new(), |mut p, (_, instruction)| {
            p.execute_instruction(instruction);
            p
        });

    println!("Part One: {}", p.x.abs() + p.y.abs());

    let p2 = p.first_repetition.unwrap();

    println!("Part Two: {}", p2.0.abs() + p2.1.abs());
}

enum Direction {
    North,
    West,
    East,
    South,
}

struct Pointer {
    x: i32,
    y: i32,
    direction: Direction,
    history: HashSet<(i32, i32)>,
    first_repetition: Option<(i32, i32)>,
}

impl Pointer {
    fn new() -> Self {
        Pointer {
            x: 0,
            y: 0,
            direction: Direction::North,
            history: HashSet::from([(0, 0)]),
            first_repetition: None,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let steps = match instruction {
            Instruction::Left(n) => n,
            Instruction::Right(n) => n,
        } as i32;

        // Turn
        self.direction = match (&self.direction, instruction) {
            (Direction::North, Instruction::Left(_)) => Direction::West,
            (Direction::West, Instruction::Left(_)) => Direction::South,
            (Direction::South, Instruction::Left(_)) => Direction::East,
            (Direction::East, Instruction::Left(_)) => Direction::North,
            (Direction::North, Instruction::Right(_)) => Direction::East,
            (Direction::East, Instruction::Right(_)) => Direction::South,
            (Direction::South, Instruction::Right(_)) => Direction::West,
            (Direction::West, Instruction::Right(_)) => Direction::North,
        };

        // Step
        for _ in 0..steps {
            self.x = match self.direction {
                Direction::West => self.x - 1,
                Direction::East => self.x + 1,
                _ => self.x,
            };

            self.y = match self.direction {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y,
            };

            // Write to history
            if self.first_repetition.is_none() {
                if self.history.contains(&(self.x, self.y)) {
                    self.first_repetition = Some((self.x, self.y));
                } else {
                    self.history.insert((self.x, self.y));
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Left(u8),
    Right(u8),
}

impl Instruction {
    fn from(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(tuple((char('L'), u8)), |(_, num)| Instruction::Left(num)),
            map(tuple((char('R'), u8)), |(_, num)| Instruction::Right(num)),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Instruction::from("L10"), Ok(("", Instruction::Left(10))));
        assert_eq!(Instruction::from("R20"), Ok(("", Instruction::Right(20))));
        assert!(Instruction::from("C20").is_err());
        assert!(Instruction::from("L-20").is_err());
        assert!(Instruction::from("L").is_err());
    }

    #[test]
    fn test_execute() {
        let mut pointer = Pointer::new();

        pointer.execute_instruction(Instruction::Left(10));
        assert_eq!(pointer.x, -10);
        assert_eq!(pointer.y, 0);

        pointer.execute_instruction(Instruction::Left(10));
        assert_eq!(pointer.x, -10);
        assert_eq!(pointer.x, -10);
    }
}
