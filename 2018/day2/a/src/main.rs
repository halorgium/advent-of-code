use std::collections::HashMap;

// abcdef contains no letters that appear exactly two or three times.
// bababc contains two a and three b, so it counts for both.
// abbcde contains two b, but no letter appears exactly three times.
// abcccd contains three c, but no letter appears exactly two times.
// aabcdd contains two a and two d, but it only counts once.
// abcdee contains two e.
// ababab contains three a and three b, but it only counts once.

fn main() {
    let input = include_str!("../input.txt");

    let checksum = process(input.to_string());
    println!("checksum={}", checksum);
}

fn process(input: String) -> u32 {
    let mut total_two_count = 0;
    let mut total_three_count = 0;

    for line in input.lines() {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for char in line.chars() {
            let counter = counts.entry(char).or_insert(0);
            *counter += 1;
        }

        let mut two_count = 0;
        let mut three_count = 0;

        for count in counts.values() {
            match count {
                2 => two_count = 1,
                3 => three_count = 1,
                _ => (),
            }
        }

        total_two_count += two_count;
        total_three_count += three_count;
    }

    total_two_count * total_three_count
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = include_str!("fixtures/simple.txt");

        assert_eq!(process(input.to_string()), 12);
    }
}
