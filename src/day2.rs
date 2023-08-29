use std::collections::HashMap;

use nom::{branch::alt, character::complete::char, combinator::map, multi::many1, IResult};

pub fn solve(input: &str) {
    let instructions: Vec<Vec<Instruction>> = input
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .map(|(_, i)| i)
        .collect();

    let mut part1 = Keyboard {
        // 1 2 3
        // 4 5 6
        // 7 8 9
        keys: HashMap::from([
            ((0, 0), '1'),
            ((1, 0), '2'),
            ((2, 0), '3'),
            ((0, 1), '4'),
            ((1, 1), '5'),
            ((2, 1), '6'),
            ((0, 2), '7'),
            ((1, 2), '8'),
            ((2, 2), '9'),
        ]),
        pointer: (1, 1),
    };

    let mut part2 = Keyboard {
        //     1
        //   2 3 4
        // 5 6 7 8 9
        //   A B C
        //     D
        keys: HashMap::from([
            ((2, 0), '1'),
            ((1, 1), '2'),
            ((2, 1), '3'),
            ((3, 1), '4'),
            ((0, 2), '5'),
            ((1, 2), '6'),
            ((2, 2), '7'),
            ((3, 2), '8'),
            ((4, 2), '9'),
            ((1, 3), 'A'),
            ((2, 3), 'B'),
            ((3, 3), 'C'),
            ((2, 4), 'D'),
        ]),
        pointer: (1, 1),
    };

    let code1: (String, &mut Keyboard) = instructions.iter().fold(
        (String::new(), &mut part1),
        |(mut code, mut keyboard), i| {
            let (keyboard, c) = keyboard.execute_instructions(i);
            code.push(c);
            (code, keyboard)
        },
    );

    println!("Part One: {}", code1.0);

    let code2: (String, &mut Keyboard) = instructions.iter().fold(
        (String::new(), &mut part2),
        |(mut code, mut keyboard), i| {
            let (keyboard, c) = keyboard.execute_instructions(i);
            code.push(c);
            (code, keyboard)
        },
    );

    println!("Part Two: {}", code2.0);
}

#[derive(Clone)]
struct Keyboard {
    keys: HashMap<(i32, i32), char>,
    pointer: (i32, i32),
}

impl Keyboard {
    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) -> (&mut Self, char) {
        for i in instructions {
            let new_key = match i {
                Instruction::Up => (self.pointer.0, self.pointer.1 - 1),
                Instruction::Down => (self.pointer.0, self.pointer.1 + 1),
                Instruction::Left => (self.pointer.0 - 1, self.pointer.1),
                Instruction::Right => (self.pointer.0 + 1, self.pointer.1),
            };

            if self.keys.contains_key(&new_key) {
                self.pointer = new_key;
            }
        }

        let field = self.keys.get(&self.pointer).unwrap().clone();

        return (self, field);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(line: &str) -> IResult<&str, Vec<Instruction>> {
    many1(Instruction::from)(line)
}

impl Instruction {
    fn from(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(char('U'), |_| Instruction::Up),
            map(char('D'), |_| Instruction::Down),
            map(char('L'), |_| Instruction::Left),
            map(char('R'), |_| Instruction::Right),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Instruction::from("U"), Ok(("", Instruction::Up)));
        assert_eq!(Instruction::from("D"), Ok(("", Instruction::Down)));
        assert_eq!(Instruction::from("L"), Ok(("", Instruction::Left)));
        assert_eq!(Instruction::from("R"), Ok(("", Instruction::Right)));
        assert!(Instruction::from("C").is_err());
    }
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("ULDR"),
            Ok((
                "",
                vec![
                    Instruction::Up,
                    Instruction::Left,
                    Instruction::Down,
                    Instruction::Right
                ]
            ))
        );
    }
}
