use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::env;
use std::fmt::{self, Write};
use std::fs;
use std::iter;
use std::ops::Deref;
use std::ops::DerefMut;
use tracing::{debug, info, trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    let level = match env::args().nth(2) {
        Some(e) => match e.as_str() {
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        },
        None => Level::INFO,
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default subscriber failed");
    info!("{input:?}");
    let input = fs::read_to_string(input)?;

    let moves: Vec<_> = input
        .lines()
        .map(|l| l.trim().split_once(' ').expect("split move"))
        .map(|(l, r)| (l, r.parse::<i32>().expect("parse to i32")))
        .collect();

    // hard coding these is a hack: segments, width, height
    let grid_size = (6, 5);
    let mut rope = Rope::new(10, grid_size);
    let mut history = History::new();

    for mov in moves {
        debug!("H:{} T:{} | {mov:?} --> ", rope.segs[0], rope.segs[1]);
        history.extend(process_move(mov, &mut rope));
        debug!("  --> H:{} T:{}", rope.segs[0], rope.segs[1]);
        //rope._plot();
    }

    //history._plot(grid_size);
    println!("part2: {}", history.iter().count());

    Ok(())
}

fn process_move(mov: (&str, i32), rope: &mut Rope) -> Vec<Position> {
    let mut history: Vec<Position> = Vec::new();
    let (dir, steps) = mov;
    for _ in 0..steps {
        match dir {
            "R" => {
                trace!("[R:{}] --> ", steps);
                rope.segs[0].0 += 1;
            }
            "L" => {
                trace!("[L:{}] --> ", steps);
                rope.segs[0].0 -= 1;
            }
            "U" => {
                trace!("[U:{}] --> ", steps);
                rope.segs[0].1 += 1;
            }
            "D" => {
                trace!("[D:{}] --> ", steps);
                rope.segs[0].1 -= 1;
            }
            _ => unreachable!(),
        };
        history.push(rope.update_tail());
    }
    history
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

    fn _plot(&self, (width, height): (usize, usize)) {
        trace!("{width}, {height}");
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..=height {
            grid.push(iter::repeat('.').take(width as usize + 1).collect());
        }
        trace!("{}, {}", grid[0].len(), grid.len());
        grid[0][0] = 's';
        for pos in &self.set {
            //println!("{pos}");
            grid[pos.1 as usize][pos.0 as usize] = '#';
        }
        for row in grid.iter().rev() {
            debug!("{}", row.iter().collect::<String>());
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
struct Rope {
    segs: Vec<Position>,
    width: usize,
    height: usize,
}

impl Rope {
    fn new(count: usize, (width, height): (usize, usize)) -> Rope {
        Rope {
            segs: iter::repeat(Position(0, 0)).take(count).collect(),
            width,
            height,
        }
    }

    fn _plot(&self) -> Result<String> {
        fn pretty(i: usize) -> char {
            match i {
                0 => 'H',
                i => char::from_digit(i as u32, 10).unwrap_or('?'),
            }
        }
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.height {
            grid.push(iter::repeat('.').take(self.width as usize).collect());
        }
        grid[0][0] = 's';
        for (i, pos) in self.segs.iter().enumerate().rev() {
            //println!("{pos}");
            grid[pos.1 as usize][pos.0 as usize] = pretty(i);
        }
        let mut output = String::new();
        for row in grid.iter().rev() {
            writeln!(output, "{}", row.iter().collect::<String>())?;
        }
        Ok(output)
    }

    fn update_tail(&mut self) -> Position {
        // wanted to iterate on .windows() here, but it wasn't available initialy and isn't mutable
        // anyway. Again, I'm probably structuring this wrong. Should this kind of function take
        // ownership and return at the end instead of a mutable borrow?
        // Also... all of this self.segs[i] indexing is a mess, must be a better way.

        for i in 0..self.segs.len() - 1 {
            let head = i;
            let tail = i + 1;
            let mut note = format!(
                "  ut:{head} H:{} T:{} --> ",
                self.segs[head], self.segs[tail]
            );

            // case: equal
            if self.segs[head] == self.segs[tail] {
                continue;
            }

            // case: vertical
            if self.segs[head].1 == self.segs[tail].1 {
                let diff = self.segs[head].0 - self.segs[tail].0;
                if diff >= 1 {
                    note.push_str("move right --> ");
                    self.segs[tail].0 += diff.abs() - 1;
                } else if diff <= -1 {
                    note.push_str("move left --> ");
                    self.segs[tail].0 -= diff.abs() - 1;
                };
            } else if self.segs[head].0 == self.segs[tail].0 {
                let diff = self.segs[head].1 - self.segs[tail].1;
                if diff >= 1 {
                    note.push_str("move up --> ");
                    self.segs[tail].1 += diff.abs() - 1;
                } else if diff <= -1 {
                    note.push_str("move down --> ");
                    self.segs[tail].1 -= diff.abs() - 1;
                };
            }

            // case: diagonal
            if (self.segs[head].0 != self.segs[tail].0) && (self.segs[head].1 != self.segs[tail].1)
            {
                let diffx = self.segs[head].0 - self.segs[tail].0;
                let diffy = self.segs[head].1 - self.segs[tail].1;
                if diffx.abs() >= 2 || diffy.abs() >= 2 {
                    if diffx > 0 && diffy > 0 {
                        note.push_str("diag up & right --> ");
                        self.segs[tail].0 += 1;
                        self.segs[tail].1 += 1;
                    } else if diffx < 0 && diffy > 0 {
                        note.push_str("diag up & left --> ");
                        self.segs[tail].0 -= 1;
                        self.segs[tail].1 += 1;
                    } else if diffx > 0 && diffy < 0 {
                        note.push_str("diag down & right --> ");
                        self.segs[tail].0 += 1;
                        self.segs[tail].1 -= 1;
                    } else if diffx < 0 && diffy < 0 {
                        note.push_str("diag down & left --> ");
                        self.segs[tail].0 -= 1;
                        self.segs[tail].1 -= 1;
                    } else {
                        //println!("H:{}, T:{}", self.segs[head], self.segs[tail]);
                        note.push_str("diag no-op--> ");
                    }
                }
            }

            note.push_str(&format!("T:{}", self.segs[tail]));
            trace!("{}", note);

            // double check: tail should never be more than one step behind
            assert!(
                (self.segs[head].0 - self.segs[tail].0).abs() <= 1,
                "update_tail check: H:{} T:{}",
                self.segs[head],
                self.segs[tail]
            );
            assert!(
                (self.segs[head].1 - self.segs[tail].1).abs() <= 1,
                "update_tail check: H:{} T:{}",
                self.segs[head],
                self.segs[tail]
            );
        }
        debug!("\n{}", self._plot().unwrap());
        self.segs[self.segs.len() - 1].clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certain_long_tail_cases() {
        let grid_size = (6, 5);
        let mut rope = Rope::new(10, grid_size);
        process_move(("R", 4), &mut rope);
        let expected = r#"
......
......
......
......
4321H.
"#;
        assert_eq!(expected.trim(), rope._plot().unwrap().trim());

        process_move(("U", 4), &mut rope);
        let expected = r#"
....H.
....1.
..432.
.5....
6.....
"#;
        println!("{}\n{}", expected.trim(), rope._plot().unwrap().trim());
        assert_eq!(expected.trim(), rope._plot().unwrap().trim());
    }
}
