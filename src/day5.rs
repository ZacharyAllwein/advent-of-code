use itertools::Itertools;
use std::io::prelude::*;

pub fn day5<T: Read>(buf: std::io::BufReader<T>) {
    //turn the file into usable data: [[(0, 1), (0,2)], etc..]
    let coordinate_lines = parse(buf);

    //filter all diagonal lines out
    let coordinate_lines: Vec<Vec<(i32, i32)>> = coordinate_lines
        .into_iter()
        .map(|vec| [vec[0], vec[1]])
        //uncomment below line for day5
        // .filter(|[(x1, y1), (x2, y2), ..]| x1 == x2 || y1 == y2)
        .map(make_line)
        .collect();

    let mut grid: [[i32; 1000]; 1000] = [[0; 1000]; 1000];

    coordinate_lines
        .into_iter()
        .for_each(|line| graph_line(line, &mut grid));

    let count = grid.iter().flatten().filter(|num| num > &&1).count();

    println!("{:?}", count);
}

fn parse<T: Read>(buf: std::io::BufReader<T>) -> Vec<Vec<(i32, i32)>> {
    buf.lines()
        .map(|line| {
            line.unwrap()
                .split(" -> ")
                .map(str::to_owned)
                .map(|pair| {
                    pair.split(',')
                        .map(|num| num.parse::<i32>().unwrap())
                        .next_tuple()
                        .unwrap()
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn make_line([(mut x1, mut y1), (mut x2, mut y2)]: [(i32, i32); 2]) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = vec![];

    if x2 - x1 != 0 && y2 - y1 != 0 {
        let x_step = (x2 - x1) / (x2 - x1).abs();
        let y_step = (y2 - y1) / (y2 - y1).abs();

        while x1 != x2 {
            line.push((x1, y1));
            x1 += x_step;
            y1 += y_step;
        }
        line.push((x1, y1));
    } else if x2 - x1 != 0 {
        let x_step = (x2 - x1) / (x2 - x1).abs();

        while x1 != x2 {
            line.push((x1, y1));
            x1 += x_step;
        }
        line.push((x1, y1));
    } else {
        let y_step = (y2 - y1) / (y2 - y1).abs();

        while y1 != y2 {
            line.push((x1, y1));
            y1 += y_step;
        }
        line.push((x1, y1));
    }

    line
}

fn graph_line(line: Vec<(i32, i32)>, graph: &mut [[i32; 1000]; 1000]) {
    for point in line {
        graph[point.1 as usize][point.0 as usize] += 1;
    }
}
