#[derive(Debug, PartialEq)]
pub enum Instruction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Instruction {
    pub(crate) fn parse(i: &str) -> Option<Vec<Self>> {
        let mut moves = Vec::new();

        for c in i.chars() {
            match c {
                'L' => moves.push(Instruction::LEFT),
                'R' => moves.push(Instruction::RIGHT),
                'U' => moves.push(Instruction::UP),
                'D' => moves.push(Instruction::DOWN),
                _ => return None,
            };
        }

        return Some(moves);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(
            Instruction::parse("UULL"),
            Some(vec!(
                Instruction::UP,
                Instruction::UP,
                Instruction::LEFT,
                Instruction::LEFT
            ))
        );
        assert_eq!(Instruction::parse("UCULL"), None);
    }
}
