use std::io::prelude::*;

pub fn part1<T: Read>(mut buf: std::io::BufReader<T>) {

    let mut states = String::new();
    
    buf.read_line(&mut states).unwrap();

    let mut states: Vec<u8> = states.split(',').map(|num| num.parse::<u8>().unwrap()).collect();

    for i in 0..256{
        let mut new_count = 0;
        states.iter().for_each(|&num| if num == 0{new_count += 1});

        states = states.into_iter().map(|num| if num == 0{6} else {num-1}).collect();
        (0..new_count).for_each(|_| states.push(8));

        println!("{:?}",i);
    }


}