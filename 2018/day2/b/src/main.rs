fn main() {
    let input = include_str!("../input.txt");

    println!("{:?}", process(input));
}

fn process(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    for line_a in &lines {
        for line_b in &lines {
            let (diff_count, matching) = char_diff(line_a, line_b);

            if diff_count == 1 {
                return Some(matching)
            }
        }
    }

    None
}

fn char_diff<'a, 'b>(left: &'a str, right: &'a str) -> (usize, String) {
    let zipped = left.chars().zip(right.chars());

    let mut diff_count = 0;
    let mut matching = String::new();

    for (l, r) in zipped {
        if l == r {
            matching.push(l);
        }
        else {
            diff_count += 1;
        }
    }

    (diff_count, matching)
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = include_str!("fixtures/simple.txt");

        assert_eq!(process(input), Some("fgij".to_string()));
    }

    #[test]
    fn char_diffs() {
        assert_eq!(char_diff("ab", "ab"), (0, "ab".to_string()));
        assert_eq!(char_diff("ab", "ac"), (1, "a".to_string()));
        assert_eq!(char_diff("ab", "bc"), (2, "".to_string()));
    }
}
