use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;
    let mut grid = Grid::parse_input(&input)?;

    println!("{grid}");

    let observe = vec![1, 2, 5, 22, 24];

    while grid.drop_to_rest() {
        if observe.contains(&grid.resting_grains) {
            println!("{grid}");
        }
    }

    println!("{grid}");
    println!("part1: {}", grid.resting_grains);
    Ok(())
}

struct Grid {
    /// (X, Y) format.
    data: HashMap<(usize, usize), Mat>,
    resting_grains: u32,
    //source: (500, 0),
    min_xy: (usize, usize),
    max_xy: (usize, usize),
    draw_buffer: usize,
}

impl Grid {
    const ORIGIN: (usize, usize) = (500, 0);

    /// drop a grain and return true if it comes to rest before the abyss
    ///
    /// A unit of sand always falls down one step if possible. If the tile immediately
    /// below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally
    /// one step down and to the left. If that tile is blocked, the unit of sand attempts to
    /// instead move diagonally one step down and to the right. Sand keeps moving as long as it is
    /// able to do so, at each step trying to move down, then down-left, then down-right. If all
    /// three possible destinations are blocked, the unit of sand comes to rest and no longer
    /// moves, at which point the next unit of sand is created back at the source.
    fn drop_to_rest(&mut self) -> bool {
        //println!("drop_to_rest()");
        let mut pos = Grid::ORIGIN;
        // always falls down one step if possible.
        'outer: while pos.1 < self.max_xy.1 {
            //println!("pos: {pos:?}");
            //if down blocked
            if self.data.contains_key(&(pos.0, pos.1 + 1)) {
                let candidates = [(pos.0 - 1, pos.1 + 1), (pos.0 + 1, pos.1 + 1)];
                if candidates
                    .iter()
                    .map(|p| self.data.contains_key(p))
                    .all(|e| e)
                {
                    //println!("inserting (at rest): ({}, {})", pos.0, pos.1);
                    assert_eq!(None, self.data.insert((pos.0, pos.1), Mat::Sand));
                    self.resting_grains += 1;
                    return true;
                }

                for candidate in candidates {
                    if !self.data.contains_key(&candidate) {
                        pos = candidate;
                        //println!("continue 'outer");
                        continue 'outer;
                    }
                }
            } else {
                pos = (pos.0, pos.1 + 1);
                continue;
            }
        }
        println!("abyss!");
        false
    }

    fn parse_input(input: &str) -> Result<Grid> {
        let mut min_x = 500;
        let mut max_x = 500;
        let mut max_y = 0;
        let mut data: HashMap<(usize, usize), Mat> = HashMap::new();
        data.insert(Grid::ORIGIN, Mat::Source);

        for line in input.lines() {
            //println!("{line:?}");
            let segments: Vec<(_, _)> = line
                .split(" -> ")
                .map(|e| e.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect();

            for seg in &segments {
                if seg.0 < min_x {
                    min_x = seg.0
                };
                if seg.0 > max_x {
                    max_x = seg.0
                };
                if seg.1 > max_y {
                    max_y = seg.1
                };
            }

            for (a, b) in segments.iter().tuple_windows() {
                //println!("{:?}, {:?}", a, b);
                if a.0 == b.0 {
                    for i in min(a.1, b.1)..=max(a.1, b.1) {
                        data.insert((a.0, i), Mat::Rock);
                    }
                } else if a.1 == b.1 {
                    for i in min(a.0, b.0)..=max(a.0, b.0) {
                        data.insert((i, a.1), Mat::Rock);
                    }
                } else {
                    panic!("unexpected segment alignment")
                }
            }
        }

        Ok(Grid {
            data,
            resting_grains: 0,
            min_xy: (min_x, 0),
            max_xy: (max_x, max_y),
            draw_buffer: 1,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let max_y = self.max_xy.1 + self.draw_buffer;
        let min_x = self.min_xy.0 - self.draw_buffer;
        let max_x = self.max_xy.0 + self.draw_buffer;
        for y in 0..=max_y {
            write!(f, "{y:>2} ")?;
            for x in min_x..=max_x {
                write!(f, "{}", self.data.get(&(x, y)).unwrap_or(&Mat::Air))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Mat {
    Air,
    Rock,
    Sand,
    Source,
}

impl Display for Mat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let icon = match self {
            Mat::Air => '.',
            Mat::Rock => '#',
            Mat::Sand => 'o',
            Mat::Source => '+',
        };
        write!(f, "{}", icon)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        println!("{}", TEST_DATA); //temp
        let mut grid = Grid::parse_input(TEST_DATA).unwrap();
        assert_eq!((494, 0), grid.min_xy);
        assert_eq!((503, 9), grid.max_xy);

        let display = r#" 0 ......+...
 1 ..........
 2 ..........
 3 ..........
 4 ....#...##
 5 ....#...#.
 6 ..###...#.
 7 ........#.
 8 ........#.
 9 #########.
"#;
        grid.draw_buffer = 0; // override for tests
        println!("expected:\n{}", display);
        println!("actual:\n{}", grid);
        assert_eq!(display, format!("{}", grid));
    }
}
