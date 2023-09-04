pub fn solve(input: &str) {
    let mut hash1 = String::new();
    let mut hash2 = [None, None, None, None, None, None, None, None];

    for i in 0.. {
        let new_hash = md5::compute(format!("{}{}", input, i));
        let hex = format!("{:x?}", &new_hash);

        if hex.starts_with("00000") {
            if hash1.len() < 8 {
                hash1.push(hex.chars().nth(5).unwrap());
                // dbg!(&hash1);
            }

            if hash2.iter().any(|c| c.is_none()) {
                let pointer = hex.chars().nth(5).unwrap();
                let value = hex.chars().nth(6).unwrap();

                match pointer.to_digit(10) {
                    Some(n) => {
                        if n < 8 {
                            // dbg!(hex, &hash2);
                            match hash2[n as usize] {
                                None => {
                                    hash2[n as usize] = Some(value);
                                }
                                Some(_) => {}
                            }
                        }
                    }
                    None => {}
                }
            }

            if hash1.len() == 8 && !hash2.iter().any(|c| c.is_none()) {
                break;
            }
        }
    }

    println!("Part One: {}", &hash1);

    println!(
        "Part Two: {}",
        hash2.map(Option::unwrap).iter().collect::<String>()
    );
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_line() {
        assert!(true);
    }
}

// You are faced with a security door designed by Easter Bunny engineers that seem to have
// acquired most of their security knowledge by watching hacking movies.

// The eight-character password for the door is generated one character
// at a time by finding the MD5 hash of some Door ID (your puzzle input) and an increasing integer index (starting with 0).

// A hash indicates the next character in the password if its hexadecimal
// representation starts with five zeroes. If it does, the sixth character in the hash is the next character of the password.

// For example, if the Door ID is abc:

//     The first index which produces a hash that starts with five zeroes is 3231929, which we find by hashing abc3231929; the sixth character of the hash, and thus the first character of the password, is 1.
//     5017308 produces the next interesting hash, which starts with 000008f82..., so the second character of the password is 8.
//     The third time a hash starts with five zeroes is for abc5278568, discovering the character f.

// In this example, after continuing this search a total of eight times, the password is 18f47a30.

// Given the actual Door ID, what is the password?

// Your puzzle input is reyedfim.
