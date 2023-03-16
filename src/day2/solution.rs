use super::parse::Instruction;

pub fn is_input_valid(input: &str) -> bool {
    input.len() > 0 && input.lines().all(|line| Instruction::parse(line).is_some())
}

pub fn solve(input: &str) -> (String, String) {
    let instructions: Vec<Vec<Instruction>> = input
        .lines()
        .map(Instruction::parse)
        .map(Option::unwrap)
        .collect();

    let part_1 = get_keyboard_input(&instructions, square_pos_to_num);
    let part_2 = get_keyboard_input(&instructions, diamond_pos_to_num);

    return (part_1, part_2);
}

fn square_pos_to_num(n: (i32, i32)) -> Option<char> {
    // 1 2 3
    // 4(5)6
    // 7 8 9

    match n {
        (-1, -1) => Some('1'),
        (0, -1) => Some('2'),
        (1, -1) => Some('3'),
        (-1, 0) => Some('4'),
        (0, 0) => Some('5'),
        (1, 0) => Some('6'),
        (-1, 1) => Some('7'),
        (0, 1) => Some('8'),
        (1, 1) => Some('9'),
        _ => None,
    }
}

fn diamond_pos_to_num(n: (i32, i32)) -> Option<char> {
    //       1
    //     2 3 4
    //  (5)6 7 8 9
    //     A B C
    //       D

    match n {
        (0, 0) => Some('5'),
        (1, -1) => Some('2'),
        (1, 0) => Some('6'),
        (1, 1) => Some('A'),
        (2, -2) => Some('1'),
        (2, -1) => Some('3'),
        (2, 0) => Some('7'),
        (2, 1) => Some('B'),
        (2, 2) => Some('D'),
        (3, -1) => Some('4'),
        (3, 0) => Some('8'),
        (3, 1) => Some('C'),
        (4, 0) => Some('9'),
        _ => None,
    }
}

fn execute_line(
    position: (i32, i32),
    line: &[Instruction],
    pos_map: fn((i32, i32)) -> Option<char>,
) -> (i32, i32) {
    let mut pos = position;
    for i in line {
        match i {
            Instruction::UP => {
                if let Some(_) = pos_map((pos.0, pos.1 - 1)) {
                    pos.1 -= 1;
                };
            }
            Instruction::DOWN => {
                if let Some(_) = pos_map((pos.0, pos.1 + 1)) {
                    pos.1 += 1;
                };
            }
            Instruction::LEFT => {
                if let Some(_) = pos_map((pos.0 - 1, pos.1)) {
                    pos.0 -= 1;
                };
            }
            Instruction::RIGHT => {
                if let Some(_) = pos_map((pos.0 + 1, pos.1)) {
                    pos.0 += 1;
                };
            }
        }
    }
    return pos;
}
fn get_positions_from_line(
    pos: &mut (i32, i32),
    line: &[Instruction],
    pos_map: fn((i32, i32)) -> Option<char>,
) -> Vec<(i32, i32)> {
    let mut position_history = Vec::new();
    for i in line {
        match i {
            Instruction::UP => {
                if let Some(_) = pos_map((pos.0, pos.1 - 1)) {
                    pos.1 -= 1;
                };
            }
            Instruction::DOWN => {
                if let Some(_) = pos_map((pos.0, pos.1 + 1)) {
                    pos.1 += 1;
                };
            }
            Instruction::LEFT => {
                if let Some(_) = pos_map((pos.0 - 1, pos.1)) {
                    pos.0 -= 1;
                };
            }
            Instruction::RIGHT => {
                if let Some(_) = pos_map((pos.0 + 1, pos.1)) {
                    pos.0 += 1;
                };
            }
        }

        position_history.push(pos.clone());
    }
    return position_history;
}

fn get_keyboard_input(
    instructions: &[Vec<Instruction>],
    pos_map: fn((i32, i32)) -> Option<char>,
) -> String {
    let mut pos = (0, 0);
    let mut digits = Vec::new();

    for line in instructions {
        for _ in line {
            pos = execute_line(pos, line, pos_map)
        }

        digits.push(pos_map(pos).unwrap());
    }

    return digits.iter().collect();
}

pub fn animation_data(input: &str) -> (Vec<Vec<(i32, i32)>>, Vec<Vec<(i32, i32)>>) {
    let instructions: Vec<Vec<Instruction>> = input
        .lines()
        .map(Instruction::parse)
        .map(Option::unwrap)
        .collect();

    let mut pos_square = (0, 0);
    let mut pos_diamond = (0, 0);
    let mut square_instructions = Vec::new();
    let mut diamond_instructions = Vec::new();

    for line in &instructions {
        square_instructions.push(get_positions_from_line(
            &mut pos_square,
            &line,
            square_pos_to_num,
        ));
        diamond_instructions.push(get_positions_from_line(
            &mut pos_diamond,
            &line,
            diamond_pos_to_num,
        ));
    }

    (square_instructions, diamond_instructions)
}

// You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a rush that you forgot to use the bathroom! Fancy office buildings like this one usually have keypad locks on their bathrooms, so you search the front desk for the code.

// "In order to improve security," the document you find says, "bathroom codes will no longer be written down. Instead, please memorize and follow the procedure below to access the bathrooms."

// The document goes on to explain that each button to be pressed can be found by starting on the previous button and moving to adjacent buttons on the keypad: U moves up, D moves down, L moves left, and R moves right. Each line of instructions corresponds to one button, starting at the previous button (or, for the first line, the "5" button); press whatever button you're on at the end of each line. If a move doesn't lead to a button, ignore it.

// You can't hold it much longer, so you decide to figure out the code as you walk to the bathroom. You picture a keypad like this:

// 1 2 3
// 4 5 6
// 7 8 9

// Suppose your instructions are:

// ULL
// RRDDD
// LURDL
// UUUUD

//     You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"), so the first button is 1.
//     Starting from the previous button ("1"), you move right twice (to "3") and then down three times (stopping at "9" after two moves and ignoring the third), ending up with 9.
//     Continuing from "9", you move left, up, right, down, and left, ending with 8.
//     Finally, you move up four times (stopping at "2"), then down once, ending with 5.

// So, in this example, the bathroom code is 1985.

// Your puzzle input is the instructions from the document you found at the front desk. What is the bathroom code?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(is_input_valid("RLUD"), true);
        assert_eq!(is_input_valid("RLCUD"), false);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("L").0, "4");
        assert_eq!(solve("LU").0, "1");
        assert_eq!(
            solve("ULL\nRRDDD\nLURDL\nUUUUD"),
            ("1985".to_string(), "5DB3".to_string())
        );
    }
}
