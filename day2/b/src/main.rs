use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn process(input: String) -> String {
    let bitmasks: HashMap<&str, usize> = input.lines().zip(input.lines())

    // println!("line={}; bitmask={}", line, bitmask);

    println!("bitmasks={:?}", bitmasks);

    // let diffs = HashMap::new();

    for line_a in {
        for line_b in &bitmasks {
            let diff = usize_diff(bitmask_a, bitmask_b);

            // let key_a = (line_a, line_b);
            // let key_b = (line_b, line_a);
            // diffs.entry(key)

            println!("lines=[{} {}] bits=[{:#b}, {:#b}] diff={}", line_a, line_b, bitmask_a, bitmask_b, diff);
        }
    }

    "".to_string()
}

fn char_diff(a: String, b: String) -> usize {
    
}

fn usize_diff(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    }
    else {
        b - a
    }
}

fn compute_bitmask(input: &str) -> usize {
    let keys: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut bitmask = 0;

    for char in input.chars() {
        let pos = keys.iter().position(|c| c == &char).expect("unable to find input char");
        println!("char={}; pos={}", char, pos);

        let bit = 1 << pos;
        if bitmask & bit > 0 {
            panic!("bit already set");
        }

        bitmask += bit;
    }

    bitmask
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = include_str!("fixtures/simple.txt");

        assert_eq!(process(input.to_string()), "fgij");
    }

    #[test]
    fn bitmask() {
        assert_eq!(compute_bitmask("abc"), 0b111);
        assert_eq!(compute_bitmask("ac"), 0b101);
    }

    #[test]
    fn char_diffs() {
        assert_eq!(char_diff("ab", "ab"), 0);
        assert_eq!(char_diff("ab", "ac"), 1);
        assert_eq!(char_diff("ab", "bc"), 1);
    }
}
