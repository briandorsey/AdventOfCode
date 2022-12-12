use color_eyre::eyre::Result;
use pathfinding::prelude::astar;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let grid = Grid::parse_input(input.as_str());
    println!("{grid}");
    //println!("{:?}\n{:?}", grid.start, grid.end);

    let part1_result = astar(
        &grid.start,
        |p| grid.successors(p),
        |p| grid.heuristic(p),
        |p| grid.check_goal(p),
    )
    .unwrap();
    println!("part 1: {:?}", part1_result.1);

    // I'll bet there is a better algorithm for this, but let's try brute force first.
    let a_starts: Vec<_> = grid
        .rows
        .iter()
        .flatten()
        .filter(|p| p.h == 0)
        .map(|p| (p.x, p.y))
        .collect();

    let mut lengths: Vec<i32> = Vec::new();

    for start in &a_starts {
        if let Some(length) = astar(
            start,
            |p| grid.successors(p),
            |p| grid.heuristic(p),
            |p| grid.check_goal(p),
        ) {
            lengths.push(length.1);
        }
    }

    println!("part 2: {:?}", lengths.iter().min().unwrap());
    Ok(())
}

type Point = (i32, i32);
type Cost = i32;

/// position
#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    /// height as integer
    h: u8,
    /// display char
    d: char,
}

fn char_to_height(c: char) -> u8 {
    match c {
        'S' => 0,
        'E' => 25,
        c => c as u8 - b'a',
    }
}

struct Grid {
    rows: Vec<Vec<Pos>>,
    start: Point,
    end: Point,
    max: Point,
}

impl Grid {
    fn check_goal(&self, point: &Point) -> bool {
        point.0 == self.end.0 && point.1 == self.end.1
    }

    fn heuristic(&self, point: &Point) -> Cost {
        (self.end.0 - point.0).abs() + (self.end.1 - point.1).abs()
    }

    fn successors(&self, point: &Point) -> Vec<(Point, Cost)> {
        let successors = vec![
            (point.0 + 1, point.1),
            (point.0 - 1, point.1),
            (point.0, point.1 + 1),
            (point.0, point.1 - 1),
        ];

        successors
            .into_iter()
            // remove out of bounds successors
            .filter(|e| (e.0 >= 0) && (e.1 >= 0) && (e.0 <= self.max.0) && (e.1 <= self.max.1))
            .filter(|e| {
                self.rows[e.1 as usize][e.0 as usize].h
                    <= self.rows[point.1 as usize][point.0 as usize].h + 1
            })
            .map(|e| (e, 1)) // hard code cost at 1
            .collect()
    }

    fn parse_input(input: &str) -> Self {
        let mut grid = Grid {
            rows: Vec::new(),
            start: (0, 0),
            end: (0, 0),
            max: (0, 0),
        };
        for (y, line) in input.lines().enumerate() {
            grid.rows.push(
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Pos {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        h: char_to_height(c),
                        d: c,
                    })
                    .inspect(|p| {
                        if p.d == 'S' {
                            grid.start = (p.x, p.y)
                        }
                    })
                    .inspect(|p| {
                        if p.d == 'E' {
                            grid.end = (p.x, p.y)
                        }
                    })
                    .collect(),
            );
        }
        grid.max = (
            (grid.rows[0].len() - 1) as i32,
            (grid.rows.len() - 1) as i32,
        );
        grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for row in &self.rows {
            writeln!(f, "{}", row.iter().map(|p| p.d).collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_char_to_height() {
        assert_eq!(0, char_to_height('S'));
        assert_eq!(25, char_to_height('E'));
        assert_eq!(0, char_to_height('a'));
        assert_eq!(25, char_to_height('z'));
    }

    #[test]
    fn test_check_goal() {
        let grid = Grid::parse_input(GRID_INPUT);
        assert!(grid.check_goal(&(grid.start.x, grid.start.y)));
        assert!(!grid.check_goal(&(grid.end.x, grid.end.y)));
        assert!(!grid.check_goal(&(42000, 42000)));
    }

    #[test]
    fn test_heuristic() {
        let grid = Grid::parse_input(GRID_INPUT);
        assert_eq!(7, grid.heuristic(&(0, 0)));

        // the actual end
        assert_eq!(0, grid.heuristic(&(5, 2)));
    }

    #[test]
    fn test_successors() {
        let grid = Grid::parse_input(GRID_INPUT);
        let result = grid.successors(&(0, 0));
        assert!(result.contains(&((0, 1), 1)), "result: {:?}", result);

        let result = grid.successors(&(3, 3));
        assert!(result.len() == 4, "result: {:?}", result);

        // lower right
        let result = grid.successors(&(7, 4));
        assert!(result.len() == 2, "result: {:?}", result);

        // off grid - todo: error
        let result = grid.successors(&(9, 9));
        assert!(result.len() == 0, "result: {:?}", result);
    }
}
