use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut freq = 0;

    for line in contents.lines() {
        println!("line={}", line);

        let n = line.parse::<i32>().unwrap();

        println!("n={:?}", n);

        freq += n;

        println!("freq={:?}", freq);
    }

    println!("final freq={:?}", freq);
}
