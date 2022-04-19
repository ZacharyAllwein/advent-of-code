use std::fs::File;
use std::io::BufReader;

mod day7;

fn main() {
    let data = File::open("data.txt").unwrap();

    let buf = BufReader::new(data);

    day7::part1(buf);
}
