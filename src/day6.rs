use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut counts: Vec<HashMap<char, u32>> = vec![HashMap::new(); 8];

    input.lines().for_each(|line| {
        line.char_indices().for_each(|(i, c)| {
            let map = counts.get_mut(i).expect("we have 6 chars");

            if let Some(x) = map.get_mut(&c) {
                *x += 1;
            } else {
                map.insert(c, 1);
            }
        })
    });

    let (part1, part2) = counts
        .iter()
        .map(|letter| {
            (
                *letter.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0,
                *letter.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().0,
            )
        })
        .fold(
            (String::new(), String::new()),
            |(mut p1, mut p2), (a, b)| {
                p1.push(a);
                p2.push(b);

                (p1, p2)
            },
        );

    println!("Part One: {}", part1);

    println!("Part Two: {}", part2);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_line() {
        assert!(true);
    }
}
