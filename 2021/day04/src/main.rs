use anyhow::Result;
use std::env;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::{prelude::*, BufReader};
use std::ops;

#[derive(Clone, Debug)]
enum BoardState {
    Active,
    WonAt(usize, usize),
}

#[derive(Clone, Debug)]
struct Board {
    data: Vec<usize>,
    width: usize,
    state: BoardState,
}

impl Board {
    fn new() -> Self {
        Board {
            data: Vec::new(),
            width: 5,
            state: BoardState::Active,
        }
    }

    fn unmarked_sum(&self, drawn: &[bool]) -> usize {
        self.data.iter().filter(|e| !drawn[**e]).sum()
    }

    fn win(&self, drawn: &[bool]) -> bool {
        // check rows
        for row in 0..self.width {
            let offset = row * self.width;
            let row = &self.data[offset..offset + self.width];
            //dbg!(row);
            if row.iter().all(|&e| drawn[e]) {
                return true;
            }
        }

        // check columns
        for index in 0..self.width {
            if self
                .data
                .iter()
                .skip(index)
                .step_by(self.width)
                .all(|&e| drawn[e])
            {
                return true;
            }
        }

        // no win condition found
        return false;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "-- {:?} -- \n", self.state)?;
        for (i, v) in self.data.iter().enumerate() {
            write!(f, "{v:>3}")?;
            if (i + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
        match self.state {
            BoardState::WonAt(num, sum) => write!(f, "{} * {} = {}", num, sum, num * sum),
            _ => Ok(()),
        }?;
        Ok(())
    }
}

fn parse_board_line(line: &str) -> Vec<usize> {
    let data: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    data
}

#[derive(Clone)]
struct Boards(Vec<Board>);

impl Boards {
    fn new() -> Self {
        Boards { 0: Vec::new() }
    }
}

impl ops::Deref for Boards {
    type Target = Vec<Board>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Boards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Boards {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Boards(\n")?;
        for b in self.iter() {
            write!(f, "{}\n", b)?;
        }
        write!(f, ")\n")?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let draws = lines.next().expect("failed to read first line")?;
    let draws: Vec<usize> = draws.split(',').map(|e| e.parse().unwrap()).collect();
    //dbg!(draws);

    let mut boards = Boards::new();
    lines.next();

    let mut staging_board = Board::new();
    for line in lines {
        //println!(" {line:?}");
        match line {
            Ok(l) if l == "" => {
                boards.push(staging_board.clone());
                staging_board.data.clear();
            }
            Ok(l) => staging_board.data.append(&mut parse_board_line(&l)),
            _ => println!("WTF?"),
        }
    }
    boards.push(staging_board.clone());

    //print!("{boards}");
    dbg! {boards.len()};

    // find the first winning boards in order
    let mut winning_boards = Boards::new();
    let mut drawn = [false; 100];
    for num in draws {
        drawn[num] = true;
        //println!("{}: {:?}", num, &drawn[..10]);
        print!("{},", num);
        for board in &mut boards.iter_mut() {
            if board.win(&drawn) {
                let sum = board.unmarked_sum(&drawn);
                board.state = BoardState::WonAt(num, sum);
                winning_boards.push(board.clone());
            }
        }
        boards.retain(|e| matches!(e.state, BoardState::Active));
    }
    println!("");

    dbg!(winning_boards.len());
    let first = &winning_boards[0];
    println!("first win: \n{}", first);

    let last = &winning_boards[winning_boards.len() - 1];
    println!("last win: \n{}", last);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board_line() {
        assert_eq!(vec![22, 13, 17, 11, 0], parse_board_line("22 13 17 11  0"));
    }

    #[test]
    fn test_board_loses() {
        let drawn = [false, true];
        let board = Board {
            data: vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
            width: 3,
        };
        assert!(!board.win(&drawn));
    }

    #[test]
    fn test_board_wins_row() {
        let drawn = [false, true];
        let board = Board {
            data: vec![0, 0, 0, 1, 1, 1, 0, 0, 0],
            width: 3,
        };
        assert!(board.win(&drawn));
    }

    #[test]
    fn test_board_wins_col() {
        let drawn = [false, true];
        let board = Board {
            data: vec![0, 1, 0, 0, 1, 0, 0, 1, 0],
            width: 3,
        };
        assert!(board.win(&drawn));
    }
}
