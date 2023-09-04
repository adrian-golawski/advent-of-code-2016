use nom::{
    character::complete::{space0, space1, u32},
    multi::separated_list0,
    IResult,
};

pub fn solve(input: &str) {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .map(|(_, i)| i)
        .collect();

    let valid_triangles = lines.iter().filter(|l| is_valid_triangle(l)).count();

    println!("Part One: {}", valid_triangles);

    let flat_lines = lines.iter().flatten();

    let (mut vertically_aligned_lines, buffer) =
        flat_lines.fold((vec![], vec![]), |(mut sorted, mut buffer), item| {
            if buffer.len() == 9 {
                sorted.push(vec![buffer[0], buffer[3], buffer[6]]);
                sorted.push(vec![buffer[1], buffer[4], buffer[7]]);
                sorted.push(vec![buffer[2], buffer[5], buffer[8]]);
                buffer = vec![];
            }
            buffer.push(*item);
            (sorted, buffer)
        });

    vertically_aligned_lines.push(vec![buffer[0], buffer[3], buffer[6]]);
    vertically_aligned_lines.push(vec![buffer[1], buffer[4], buffer[7]]);
    vertically_aligned_lines.push(vec![buffer[2], buffer[5], buffer[8]]);

    dbg!(vertically_aligned_lines.len());

    let valid_triangles = vertically_aligned_lines
        .iter()
        .filter(|&l| is_valid_triangle(l))
        .count();

    println!("Part Two: {}", valid_triangles);
}

fn is_valid_triangle(l: &[u32]) -> bool {
    l[0] + l[1] > l[2] && l[0] + l[2] > l[1] && l[1] + l[2] > l[0]
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?;
    separated_list0(space1, u32)(input)
}

// Now that you've helpfully marked up their design documents, it occurs to you that triangles are specified in groups of three vertically.
// Each set of three numbers in a column specifies a triangle. Rows are unrelated.

// For example, given the following specification, numbers with the same hundreds digit would be part of the same triangle:

// 101 301 501
// 102 302 502
// 103 303 503
// 201 401 601
// 202 402 602
// 203 403 603

// In your puzzle input, and instead reading by columns, how many of the listed triangles are possible?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(" 2  628  436"), Ok(("", vec![2, 628, 436])));
    }

    #[test]
    fn test_is_valid_triangle() {
        assert!(is_valid_triangle(&vec![10, 10, 10]));
        assert!(!is_valid_triangle(&vec![1, 1, 10]));
    }
}
