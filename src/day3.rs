use std::io::prelude::*;

pub fn part2<T: std::io::Read>(buf: std::io::BufReader<T>) {
    let mut most_common_bits: Vec<Vec<u8>> =
        buf.lines().map(|line| line.unwrap().into_bytes()).collect();
    let mut least_common_bits: Vec<Vec<u8>> = most_common_bits.clone();

    for i in 0..most_common_bits[0].len() {
        if most_common_bits.len() <= 1 {
            break;
        }

        let zero_count = most_common_bits.iter().filter(|line| line[i] == 48).count();

        let mut filter_key = 49;

        if zero_count > most_common_bits.len() / 2 {
            filter_key = 48;
        }

        most_common_bits = most_common_bits
            .into_iter()
            .filter(|line| line[i] == filter_key)
            .collect();
    }

    for i in 0..least_common_bits[0].len() {
        if least_common_bits.len() <= 1 {
            break;
        }

        let zero_count = least_common_bits
            .iter()
            .filter(|line| line[i] == 48)
            .count();

        let mut filter_key = 48;

        if zero_count > least_common_bits.len() / 2 {
            filter_key = 49;
        }

        least_common_bits = least_common_bits
            .into_iter()
            .filter(|line| line[i] == filter_key)
            .collect();
    }

    //working

    let least_common_bits: Vec<u8> = least_common_bits.into_iter().flatten().collect();
    let most_common_bits: Vec<u8> = most_common_bits.into_iter().flatten().collect();

    println!("{:?}", most_common_bits);

    let mut most_common_bits_num = 0;
    let mut least_common_bits_num = 0;

    for (i, num) in most_common_bits.into_iter().rev().enumerate() {
        if num == 49 {
            most_common_bits_num += 2i32.pow(i as u32);
        }
    }
    for (i, num) in least_common_bits.into_iter().rev().enumerate() {
        if num == 49 {
            least_common_bits_num += 2i32.pow(i as u32);
        }
    }

    let x = least_common_bits_num * most_common_bits_num;

    println!("{}", x);
}
