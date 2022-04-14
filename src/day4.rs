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
        match find_winners(&boards) {
            None => (),
            Some(board) => {
                println!("{:?}", calculate_score(&board, num));
                break;
            }
        }
    }
}
pub fn part2<T: std::io::Read>(buf: std::io::BufReader<T>) {
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

    let mut winner_indexes: Vec<usize> = vec![];

    for num in bingo_seq {
        for i in 0..boards.len() {
            let board = &boards[i];
            for row in board {
                for square in row {
                    if num == square.borrow().num {
                        square.borrow_mut().marked = true;
                    }
                }
            }

            if wins(&board) {
                if !winner_indexes.contains(&(i as usize)) {
                    winner_indexes.push(i);
                }
            }
        }
        // println!("{} {}", winner_indexes.len(), boards.len());
        if winner_indexes.len() == boards.len() {
            let last_board = &boards[*winner_indexes.last().unwrap()];
            println!("{}", calculate_score(&last_board, num))
        }
    }
}

fn wins(board: &Vec<Vec<RefCell<BingoSquare>>>) -> bool {
    let mut wincons: Vec<Vec<(u8, u8)>> =
        (0..5).map(|x| (0..5).map(|y| (x, y)).collect()).collect();
    let mut vertical_wincons: Vec<Vec<(u8, u8)>> =
        (0..5).map(|x| (0..5).map(|y| (y, x)).collect()).collect();

    wincons.append(&mut vertical_wincons);

    for wincon in &wincons {
        let mut won = true;

        for con in wincon {
            if board[con.0 as usize][con.1 as usize].borrow().marked == false {
                won = false;
                break;
            }
        }

        if won {
            return true;
        }
    }

    false
}

fn find_winners(
    boards: &Vec<Vec<Vec<RefCell<BingoSquare>>>>,
) -> Option<&Vec<Vec<RefCell<BingoSquare>>>> {
    let mut wincons: Vec<Vec<(u8, u8)>> =
        (0..5).map(|x| (0..5).map(|y| (x, y)).collect()).collect();
    let mut vertical_wincons: Vec<Vec<(u8, u8)>> =
        (0..5).map(|x| (0..5).map(|y| (y, x)).collect()).collect();

    wincons.append(&mut vertical_wincons);

    for board in boards {
        for wincon in &wincons {
            let mut won = true;

            for con in wincon {
                if board[con.0 as usize][con.1 as usize].borrow().marked == false {
                    won = false;
                    break;
                }
            }

            if won == true {
                return Some(board);
            }
        }
    }

    None
}

fn calculate_score(board: &Vec<Vec<RefCell<BingoSquare>>>, last_num: u8) -> u32 {
    board
        .iter()
        .map(|row| {
            row.iter().fold(0, |total, square| {
                let square = square.borrow();
                if !square.marked {
                    return total + square.num as u32;
                }
                total
            })
        })
        .fold(0, |total, row_sum| total + row_sum as u32)
        * last_num as u32
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
