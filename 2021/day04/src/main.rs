use anyhow::Result;
use std::env;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::{prelude::*, BufReader};
use std::ops;

#[derive(Clone, Debug)]
struct Board {
    data: Vec<usize>,
    width: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            data: Vec::new(),
            width: 5,
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
        //write!(f, "----\n")?;
        for (i, v) in self.data.iter().enumerate() {
            write!(f, "{v:>3}")?;
            if (i + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
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

    let mut boards = Boards { 0: Vec::new() };
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

    let mut drawn = [false; 100];
    'outer: for num in draws {
        drawn[num] = true;
        //println!("{}: {:?}", num, &drawn[..10]);
        print!("{},", num);
        for board in boards.iter() {
            if board.win(&drawn) {
                println!("\n{board}\n wins!");
                let sum = board.unmarked_sum(&drawn);
                println!("{} * {} = {}", sum, num, sum * num);
                break 'outer;
            }
        }
    }
    println!("");

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
