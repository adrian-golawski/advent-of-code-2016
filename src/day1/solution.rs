use std::collections::HashSet;

use super::parse::{Instruction, Turn};

pub fn is_input_valid(str: &str) -> bool {
    str.split(", ").map(Instruction::parse).all(|x| x.is_ok())
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let instructions: Vec<Instruction> = input
        .split(", ")
        .map(Instruction::parse)
        .map(Result::unwrap)
        .map(|p| p.1)
        .collect();

    let mut position = Position::new();

    for i in instructions {
        position.execute_instruction(&i);
    }

    let part_1 = position.coordinates.0.abs() + position.coordinates.1.abs();
    let first_repetition = position.find_first_repetition();

    if let Some((x, y)) = first_repetition {
        let part_2 = x.abs() + y.abs();
        return Some((part_1, part_2));
    }

    None
}

pub fn animation_data(input: &str) -> (Vec<(i32, i32)>, Option<(i32, i32)>) {
    let instructions: Vec<Instruction> = input
        .split(", ")
        .map(Instruction::parse)
        .map(Result::unwrap)
        .map(|p| p.1)
        .collect();

    let mut position = Position::new();

    for i in instructions {
        position.execute_instruction(&i);
    }

    let first_repetition = position.find_first_repetition();

    return (position.history, first_repetition);
}

#[derive(Debug)]
struct Position {
    direction: Direction,
    coordinates: (i32, i32),
    history: Vec<(i32, i32)>,
}

impl Position {
    fn new() -> Position {
        Position {
            direction: Direction::North,
            coordinates: (0, 0),
            history: vec![(0, 0)],
        }
    }

    fn turn(&mut self, instruction: &Instruction) -> &mut Self {
        self.direction = match self.direction {
            Direction::North => match instruction.turn {
                Turn::Right => Direction::East,
                Turn::Left => Direction::West,
            },
            Direction::South => match instruction.turn {
                Turn::Right => Direction::West,
                Turn::Left => Direction::East,
            },
            Direction::East => match instruction.turn {
                Turn::Right => Direction::South,
                Turn::Left => Direction::North,
            },
            Direction::West => match instruction.turn {
                Turn::Right => Direction::North,
                Turn::Left => Direction::South,
            },
        };

        return self;
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> &mut Self {
        self.turn(&instruction);

        for _i in 0..instruction.distance {
            let (x, y) = self.coordinates;
            self.coordinates = match &self.direction {
                // Remmeber: 0,0 is at North-West
                Direction::North => (x, y + 1),
                Direction::South => (x, y - 1),
                Direction::East => (x + 1, y),
                Direction::West => (x - 1, y),
            };
            self.history.push(self.coordinates);
        }

        return self;
    }

    fn find_first_repetition(&self) -> Option<(i32, i32)> {
        let mut history = HashSet::new();

        for h in &self.history {
            if history.contains(&h) {
                return Some(*h);
            } else {
                history.insert(h);
            }
        }

        None
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(is_input_valid("R1, L2, L10"), true);
        assert_eq!(is_input_valid("V1, L2, L10"), false);
        assert_eq!(is_input_valid("VC, L2, L10"), false);
        assert_eq!(is_input_valid("VC, L2, 0"), false);
    }
}
