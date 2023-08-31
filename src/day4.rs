use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

pub fn solve(input: &str) {
    let lines: Vec<Line> = input
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .map(|i| i.1)
        .collect();

    let verified: Vec<&Line> = lines.iter().filter(|&l| verify_line(l)).collect();

    let part1 = verified.iter().map(|l| l.sector).sum::<u32>();

    println!("Part One: {}", part1);

    let decoded: &&Line = verified
        .iter()
        .find(|l| rotate_by_sector(&l).eq("northpole object storage"))
        .unwrap();

    println!("Part Two: {}", decoded.sector);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Line<'a> {
    encoded_name: Vec<&'a str>,
    sector: u32,
    checksum: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, name) = separated_list1(tag("-"), alpha1)(input)?;
    let (input, sector) = preceded(tag("-"), u32)(input)?;
    let (input, checksum) = delimited(tag("["), alpha1, tag("]"))(input)?;

    Ok((
        input,
        Line {
            encoded_name: name,
            sector,
            checksum,
        },
    ))
}

fn verify_line(line: &Line) -> bool {
    let mut letters: HashMap<char, u32> = HashMap::new();

    line.encoded_name
        .join("")
        .chars()
        .for_each(|c| match letters.get_mut(&c) {
            Some(count) => {
                *count += 1;
            }
            None => {
                letters.insert(c, 1);
            }
        });

    let mut counts: Vec<(u32, char)> = letters.iter().map(|(c, num)| (*num, *c)).collect();

    counts.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => a.1.cmp(&b.1),
        Ordering::Greater => Ordering::Less,
    });

    let checksum = counts.iter().map(|(_, c)| *c).take(5).collect::<String>();

    checksum.eq(&line.checksum)
}

fn rotate_by_sector(l: &Line) -> String {
    let name = l.encoded_name.join(" ");

    let name = name
        .bytes()
        .map(|b| match b {
            b' ' => ' ',
            _ => ((b as u32 + l.sector - 97) % 26 + 97) as u8 as char,
        })
        .collect::<String>();

    return name;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("aaaaa-bbb-z-y-x-123[abxyz]"),
            Ok((
                "",
                Line {
                    encoded_name: vec!["aaaaa", "bbb", "z", "y", "x"],
                    sector: 123,
                    checksum: "abxyz"
                }
            ))
        );
    }

    #[test]
    fn test_verify_line() {
        assert_eq!(
            verify_line(&Line {
                encoded_name: vec!["aaaaa", "bbb", "z", "y", "x"],
                sector: 123,
                checksum: "abxyz"
            }),
            true
        );

        assert_eq!(
            verify_line(&parse_line("a-b-c-d-e-f-g-h-987[abcde]").unwrap().1),
            true
        );

        assert_eq!(
            verify_line(&parse_line("not-a-real-room-404[oarel]").unwrap().1),
            true
        );

        assert_eq!(
            verify_line(&parse_line("totally-real-room-200[decoy]").unwrap().1),
            false
        );
    }

    #[test]
    fn verify_rotate() {
        dbg!(b'a' - 97);
        dbg!(b'z' - b'a');
        assert_eq!(
            rotate_by_sector(&Line {
                encoded_name: vec!["qzmt", "zixmtkozy", "ivhz"],
                sector: 343,
                checksum: "abxyz"
            }),
            String::from("very encrypted name")
        );
    }
}
