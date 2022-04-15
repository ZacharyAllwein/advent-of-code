use std::collections::HashMap;
use std::io::prelude::*;

pub fn part1<T: Read>(mut buf: std::io::BufReader<T>) {
    let mut states = String::new();
    buf.read_line(&mut states).unwrap();

    let mut states: Vec<u8> = states
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    for i in 0..18 {
        let mut new_count = 0;
        states.iter().for_each(|&num| {
            if num == 0 {
                new_count += 1
            }
        });

        states = states
            .into_iter()
            .map(|num| if num == 0 { 6 } else { num - 1 })
            .collect();
        (0..new_count).for_each(|_| states.push(8));

        println!("{:?}", i);
    }
}

pub fn part2<T: Read>(mut buf: std::io::BufReader<T>) {
    let mut states = String::new();
    buf.read_line(&mut states).unwrap();

    let states: Vec<u8> = states
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    let mut states_counter: HashMap<u8, u64> = HashMap::new();

    (0..9).for_each(|num| {
        states_counter.insert(num as u8, 0);
    });

    states.iter().for_each(|num| {
        *states_counter.get_mut(num).unwrap() += 1;
    });

    for _ in 0..80 {
        let zero_count = *states_counter.get(&0).unwrap();

        //its umm ultra performant maybe
        *states_counter.get_mut(&0).unwrap() = *states_counter.get_mut(&1).unwrap();
        *states_counter.get_mut(&1).unwrap() = *states_counter.get_mut(&2).unwrap();
        *states_counter.get_mut(&2).unwrap() = *states_counter.get_mut(&3).unwrap();
        *states_counter.get_mut(&3).unwrap() = *states_counter.get_mut(&4).unwrap();
        *states_counter.get_mut(&4).unwrap() = *states_counter.get_mut(&5).unwrap();
        *states_counter.get_mut(&5).unwrap() = *states_counter.get_mut(&6).unwrap();
        *states_counter.get_mut(&6).unwrap() = *states_counter.get_mut(&7).unwrap() + zero_count;
        *states_counter.get_mut(&7).unwrap() = *states_counter.get_mut(&8).unwrap();
        *states_counter.get_mut(&8).unwrap() = zero_count;
    }

    let count = states_counter
        .into_iter()
        .fold(0, |total, (_, count)| total + count);
    println!("{:?}", count);
}
