use std::fs::File;
use std::io::BufReader;

mod day6;

fn main() {
    let data = File::open("data.txt").unwrap();

    let buf = BufReader::new(data);

    day6::part2(buf);
}
