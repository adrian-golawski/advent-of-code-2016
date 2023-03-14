pub fn is_input_valid(_str: &str) -> bool {
    false
}

pub fn solve(_input: &str) -> Option<(i32, i32)> {
    Some((0, 0))
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
