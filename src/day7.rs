use std::io::prelude::*;

pub fn part1<T: Read>(mut buf: std::io::BufReader<T>) {
    let mut data = String::new();
    buf.read_line(&mut data).unwrap();

    let data: Vec<u32> = data.split(',').map(|num| num.parse::<u32>().unwrap()).collect();

    // let distance_from = |num| {
    //     data.iter().fold(0, |total, &current| total + (current as i32 - num as i32).abs())
    // };

    let distance_from_part2 = |num| {
        data.iter().fold(0, |total, &current| total + part2_calculate_distance((current as i32 - num as i32).abs() as u32, 0))
    };


    let mut range = (0, 1000);

    while range.0 != range.1{
        let (distance1, distance2) = (distance_from_part2(range.0), distance_from_part2(range.1));

        let step = ((range.1 - range.0) as f32 / 2.0).ceil() as u32;

        if distance1 < distance2{
            range.1 -= step;
        }
        else if distance2 < distance1{
            range.0 += step;
        }
        else{
            range.0 += 1;
            range.1 -= 1;
        }

        println!("{:?}", range);
    }
    println!("{}", distance_from_part2(range.0));
}

fn part2_calculate_distance(mut range: u32, mut total: u32) -> u32{
    if range == 0{
        return total;
    }
    total += range;
    range -= 1;
    part2_calculate_distance(range, total)
}