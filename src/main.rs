use std::fs::File;
use std::io::{prelude::*, BufReader};

mod day3;

fn main() {
    let data = File::open("data.txt").unwrap();

    let buf = BufReader::new(data);

    let lines: Vec<Vec<u8>> = buf.lines().map(|line| line.unwrap().into_bytes()).collect();

    day3::part2(lines);
}
