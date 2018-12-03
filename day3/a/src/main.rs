extern crate regex;
use std::collections::HashMap;
use regex::{Regex, Match};

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", calculate_overlap(input));
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    origin: (u32, u32),
    size: (u32, u32),
}

impl Claim {
    fn covered_points(&self) -> Vec<(u32, u32)> {
        let mut points = Vec::new();

        let mut x = self.origin.0;
        while x < self.origin.0 + self.size.0 {
            let mut y = self.origin.1;
            while y < self.origin.1 + self.size.1 {
                points.push((x, y));
                y += 1;
            }
            x += 1;
        }

        points
    }
}

fn calculate_overlap(input: &str) -> usize {
    let claims: Vec<Claim> = input.lines().map(|line| parse_line(&line)).collect();
    // println!("claims={:?}", claims);

    let mut fabric: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in claims {
        let points = claim.covered_points();
        // println!("points={:?}", points);

        for (x, y) in points {
            // println!("({}, {})", x, y);
            let entry = fabric.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    }

    // println!("fabric={:?}", fabric);

    let count = fabric.values().filter(|v| *v > &1).count();

    count
}

fn parse_line(content: &str) -> Claim {
    let re: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();

    let captures = re.captures(content).unwrap();

    Claim {
        id: extract_u32(captures.name("id")),
        origin: (extract_u32(captures.name("x")), extract_u32(captures.name("y"))),
        size: (extract_u32(captures.name("w")), extract_u32(captures.name("h"))),
    }
}

fn extract_u32(value: Option<Match>) -> u32 {
    value.unwrap().as_str().parse().unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = include_str!("fixtures/simple.txt");

        assert_eq!(calculate_overlap(input), 4);
    }

    #[test]
    fn parse() {
        let expected = Claim {
            id: 3,
            origin: (42, 91),
            size: (100, 58),
        };
        assert_eq!(parse_line("#3 @ 42,91: 100x58"), expected);
    }
}
