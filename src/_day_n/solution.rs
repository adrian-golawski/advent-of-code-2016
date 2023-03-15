pub fn is_input_valid(_str: &str) -> bool {
    true
}

pub fn solve(_input: &str) -> Option<()> {
    Some(())
}

pub fn animation_data(_input: &str) -> Option<()> {
    Some(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(is_input_valid("example"), true);
    }
}
