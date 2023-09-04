use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map, multi::many1,
    sequence::delimited, IResult,
};

pub fn solve(input: &str) {
    let lines: Vec<Vec<Piece>> = input.lines().flat_map(parse_line).map(|l| l.1).collect();

    let valid = lines.iter().filter(|line| is_valid_part_1(line)).count();
    println!("Part One: {}", valid);

    let valid = lines.iter().filter(|line| is_valid_part_2(line)).count();
    println!("Part Two: {}", valid);
}

#[derive(Debug, PartialEq, Eq)]
enum Piece<'a> {
    Open(&'a str),
    Brackets(&'a str),
}

fn is_valid_part_1(line: &[Piece<'_>]) -> bool {
    line.iter().any(|p| match p {
        Piece::Open(s) => contains_abba(s),
        Piece::Brackets(_) => false,
    }) && line.iter().all(|p| match p {
        Piece::Open(_) => true,
        Piece::Brackets(s) => !contains_abba(s),
    })
}

fn is_valid_part_2(line: &[Piece<'_>]) -> bool {
    let result: Vec<String> = line
        .iter()
        .filter_map(|p| match p {
            Piece::Open(s) => contains_aba(s),
            Piece::Brackets(_) => None,
        })
        .flatten()
        .collect();

    result.iter().any(|pattern| {
        line.iter().any(|p| match p {
            Piece::Open(_) => false,
            Piece::Brackets(s) => s.contains(pattern),
        })
    })
}

fn contains_abba(input: &str) -> bool {
    for i in 0..=(input.len() - 4) {
        let (a, b, c, d) = (
            input.chars().nth(i).unwrap(),
            input.chars().nth(i + 1).unwrap(),
            input.chars().nth(i + 2).unwrap(),
            input.chars().nth(i + 3).unwrap(),
        );

        if a == d && b == c && a != b {
            return true;
        }
    }

    false
}

fn contains_aba(input: &str) -> Option<Vec<String>> {
    let mut patterns: Vec<String> = Vec::new();
    for i in 0..=input.len() - 3 {
        let (a, b, c) = (
            input.chars().nth(i).unwrap(),
            input.chars().nth(i + 1).unwrap(),
            input.chars().nth(i + 2).unwrap(),
        );

        if a == c && a != b {
            patterns.push([b, a, b].iter().collect());
        }
    }

    if patterns.is_empty() {
        None
    } else {
        Some(patterns)
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Piece>> {
    many1(alt((
        map(delimited(tag("["), alpha1, tag("]")), Piece::Brackets),
        map(alpha1, Piece::Open),
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "wysextplwqpvipxdv[srzvtwbfzqtspxnethm]syqbzgtboxxzpwr[kljvjjkjyojzrstfgrw]obdhcczonzvbfby[svotajtpttohxsh]cooktbyumlpxostt";

        assert_eq!(
            parse_line(input).unwrap().1,
            vec![
                Piece::Open("wysextplwqpvipxdv"),
                Piece::Brackets("srzvtwbfzqtspxnethm"),
                Piece::Open("syqbzgtboxxzpwr"),
                Piece::Brackets("kljvjjkjyojzrstfgrw"),
                Piece::Open("obdhcczonzvbfby"),
                Piece::Brackets("svotajtpttohxsh"),
                Piece::Open("cooktbyumlpxostt"),
            ]
        );

        assert_eq!(
            parse_line("abba[mnop]qrst").unwrap().1,
            vec![
                Piece::Open("abba"),
                Piece::Brackets("mnop"),
                Piece::Open("qrst")
            ]
        );
    }

    #[test]
    fn test_contains_abba() {
        assert_eq!(contains_abba("abba"), true);
    }
    #[test]
    fn test_is_valid() {
        assert!(is_valid_part_1(&parse_line("abba[mnop]qrst").unwrap().1));
        assert!(!is_valid_part_1(&parse_line("abcd[bddb]xyyx").unwrap().1));
        assert!(!is_valid_part_1(&parse_line("aaaa[qwer]tyui").unwrap().1));
        assert!(is_valid_part_1(
            &parse_line("iboxxoj[asdfgh]zxcvbn").unwrap().1
        ));
        assert!(!is_valid_part_1(&parse_line("wysextplwqpvipxdv[srzvtwbfzqtspxnethm]syqbzgtboxxzpwr[kljvjjkjyojzrstfgrw]obdhcczonzvbfby[svotajtpttohxsh]cooktbyumlpxostt").unwrap().1));
    }
}
