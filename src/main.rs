use std::fs::File;
use std::io::BufReader;

mod day5;

fn main() {
    let data = File::open("data.txt").unwrap();

    let buf = BufReader::new(data);

    day5::part1(buf);
}
