use std::io::prelude::*;

pub fn part1<T: Read>(mut buf: std::io::BufReader<T>) {
    let mut data = String::new();
    buf.read_line(&mut data).unwrap();

    let data: Vec<u32> = data.split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    let distance_from = |num| {
        data.iter().fold(0, |total, &current| total + (current as i32 - num as i32).abs())
    };

    let range = (0, 1000);

    loop {
        let range_spread = range.1 - range.0;

        if range_spread % 2 == 0{
            
        }
        let (distance1, distance2) = (distance_from(range.0), distance_from(range.1));
    }
}