use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::iter;
use std::ops::Deref;
use std::ops::DerefMut;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let moves: Vec<_> = input
        .lines()
        .map(|l| l.trim().split_once(' ').expect("split move"))
        .map(|(l, r)| (l, r.parse::<i32>().expect("parse to i32")))
        .collect();

    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    let mut history = History::new();

    for mov in moves {
        print!("H:{head} T:{tail} | {mov:?} --> ");
        match mov {
            ("R", steps) => {
                println!("[R:{}] --> ", steps);
                for _ in 0..steps {
                    head.0 += 1;
                    update_tail(&head, &mut tail, &mut history);
                }
            }
            ("L", steps) => {
                println!("[L:{}] --> ", steps);
                for _ in 0..steps {
                    head.0 -= 1;
                    update_tail(&head, &mut tail, &mut history);
                }
            }
            ("U", steps) => {
                println!("[U:{}] --> ", steps);
                for _ in 0..steps {
                    head.1 += 1;
                    update_tail(&head, &mut tail, &mut history);
                }
            }
            ("D", steps) => {
                println!("[D:{}] --> ", steps);
                for _ in 0..steps {
                    head.1 -= 1;
                    update_tail(&head, &mut tail, &mut history);
                }
            }
            (&_, _) => unreachable!(),
        }
        println!("  --> H:{head} T:{tail}");
    }

    //println!("part1: {}", history);
    //history.plot();
    println!("part1: {}", history.iter().count());

    Ok(())
}

struct History {
    set: HashSet<Position>,
}

impl History {
    fn new() -> History {
        History {
            set: HashSet::new(),
        }
    }

    fn plot(&self) {
        if let (Some(width), Some(height)) = (
            self.set.iter().map(|e| e.0).max(),
            self.set.iter().map(|e| e.1).max(),
        ) {
            //println!("{width}, {height}");
            let mut grid: Vec<Vec<char>> = Vec::new();
            for _ in 0..=height {
                grid.push(iter::repeat('.').take(width as usize + 1).collect());
            }
            //println!("{}, {}", grid[0].len(), grid.len());
            grid[0][0] = 's';
            for pos in &self.set {
                //println!("{pos}");
                grid[pos.1 as usize][pos.0 as usize] = '#';
            }
            for row in grid.iter().rev() {
                println!("{}", row.iter().collect::<String>());
            }
        }
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for item in &self.set {
            write!(f, "{}, ", item)?;
        }
        Ok(())
    }
}

impl Deref for History {
    type Target = HashSet<Position>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.set
    }
}
impl DerefMut for History {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.set
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position(i32, i32);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({},{})", self.0, self.1)?;
        Ok(())
    }
}

fn update_tail_inline(head: &Position, tail: &mut Position) {
    // case: directly in-line from each other
    if head.1 == tail.1 {
        if head.0 - tail.0 >= 1 {
            //print!("move right --> ");
            tail.0 += 1;
        } else if head.0 - tail.0 <= 1 {
            //print!("move left --> ");
            tail.0 -= 1;
        };
    } else if head.0 == tail.0 {
        if head.1 - tail.1 >= 1 {
            //print!("move up --> ");
            tail.1 += 1;
        } else if head.1 - tail.1 <= 1 {
            //print!("move down --> ");
            tail.1 -= 1;
        };
    }
}
fn update_tail(head: &Position, tail: &mut Position, history: &mut History) {
    //print!("  ut: H:{head} T:{tail} --> ");

    // case: diagonal
    if !(head.0 == tail.0 && head.1 == tail.1) {
        if (head.0 - tail.0).abs() >= 2 {
            //print!("align vertical --> ");
            tail.1 = head.1;
            update_tail_inline(head, tail);
        } else if (head.1 - tail.1).abs() >= 2 {
            //print!("align horizontal --> ");
            tail.0 = head.0;
            update_tail_inline(head, tail);
        }
    }

    //println!("{tail}");
    // double check: tail should never be more than one step behind
    assert!(
        !((head.0 - tail.0).abs() > 1),
        "update_tail check: H:{head} T:{tail}",
    );
    assert!(
        !((head.1 - tail.1).abs() > 1),
        "update_tail check: H:{head} T:{tail}",
    );
    history.insert(tail.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
