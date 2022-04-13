use std::cell::RefCell;
use std::io::prelude::*;

pub fn part1<T: std::io::Read>(buf: std::io::BufReader<T>) {
    let mut lines = buf.lines().map(|line| line.unwrap());

    let bingo_seq = lines.next().unwrap();

    let bingo_seq: Vec<u8> = bingo_seq
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    let mut lines = lines.filter(|line| line != "");

    let mut boards: Vec<Vec<Vec<RefCell<BingoSquare>>>> = vec![];

    loop {
        let rows = (&mut lines).take(5);

        let mut rows = rows.peekable();

        if let None = rows.peek() {
            break;
        }

        boards.push(
            rows.take(5)
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| RefCell::new(BingoSquare::new(num.parse::<u8>().unwrap())))
                        .collect::<Vec<RefCell<BingoSquare>>>()
                })
                .collect::<Vec<Vec<RefCell<BingoSquare>>>>(),
        );
    }

    for num in bingo_seq {
        for board in &boards {
            for row in board {
                for square in row {
                    if num == square.borrow().num {
                        square.borrow_mut().marked = true;
                    }
                }
            }
        }
    }

    println!("{:?}", boards);
}

fn find_winners(boards: &Vec<Vec<Vec<RefCell<BingoSquare>>>>) -> Option<Vec<RefCell<BingoSquare>>> {
}

#[derive(Debug)]
struct BingoSquare {
    num: u8,
    marked: bool,
}

impl BingoSquare {
    fn new(num: u8) -> BingoSquare {
        BingoSquare { num, marked: false }
    }
}
