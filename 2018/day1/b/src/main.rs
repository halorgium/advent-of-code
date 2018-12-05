use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    process(contents);
}

fn process(contents: String) {
    let mut freq = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut seen_twice = None;

    while seen_twice.is_none() {
        for line in contents.lines() {
            println!("line={}", line);

            let n = line.parse::<i32>().unwrap();

            println!("n={:?}", n);

            freq += n;

            println!("freq={:?}", freq);

            if seen.contains(&freq) {
                println!("found freq={:?}", freq);
                seen_twice = Some(freq);
                break;
            }

            seen.insert(freq);
        }
    }

    println!("seen_twice={:?}", seen_twice);
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let contents = "+1
-2
+3
+1";
        process(contents.to_string());
    }
}
