use std::fs::File;
use std::io::BufReader;

mod day4;

fn main() {
    let data = File::open("data.txt").unwrap();

    let buf = BufReader::new(data);

    day4::part2(buf);
}
