//use color_eyre::eyre::Result;
use color_eyre::eyre::{eyre, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let grid = Grid::new(&input)?;
    let mut count_visible = 0;
    for y in 0..grid.data.len() {
        for x in 0..grid.data[0].len() {
            // assuming rectangular grid

            if grid.visible((x, y)) {
                count_visible += 1;
            }
        }
    }
    println!("part1: {count_visible}");

    Ok(())
}

/// Grid of integers with 0,0 at upper left
struct Grid {
    data: Vec<Vec<u32>>,
}

impl Grid {
    fn new(raw: &str) -> Result<Grid> {
        let mut grid = Grid::default();
        for line in raw.lines() {
            let row = line
                .trim()
                .chars()
                .map(|e| e.to_digit(10))
                .into_iter()
                .collect::<Option<Vec<u32>>>()
                .ok_or(eyre!("failure parsing grid"))?;
            grid.data.push(row);
        }
        Ok(grid)
    }

    fn value_at(&self, coords: (usize, usize)) -> u32 {
        let (x, y) = coords;
        //println!("{coords:?}: {}", self.data[y][x]);
        // oposite of what you might expect
        self.data[y][x]
    }

    fn visible(&self, coords: (usize, usize)) -> bool {
        let height = self.value_at(coords);
        let (x, y) = coords;

        let mut segments: Vec<&[u32]> = Vec::new();
        segments.push(&self.data[y][..x]); // above coords
        segments.push(&self.data[y][x + 1..]); // below coords
        let binding: Vec<_> = self
            .data
            .iter()
            .map(|e| {
                e.iter()
                    .enumerate()
                    .filter_map(|(i, v)| if i == x { Some(*v) } else { None })
            })
            .flatten()
            .collect();
        segments.push(&binding[..y]); // left of coords
        segments.push(&binding[y + 1..]); // right of coords
                                          //println!("V: {:?}", height);
                                          //println!("V: {:?}", segments);
                                          //println!( "V: {:?}", segments .iter() .map(|seg| seg.iter().all(|e| e < &height)) .collect::<Vec<bool>>());
        segments.iter().any(|seg| seg.iter().all(|e| e < &height))
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid { data: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_true() {
        println!("{:?}", DATA);
        assert!(true);
    }

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(DATA).unwrap();
        assert_eq!(5, grid.data.len());
        assert_eq!(5, grid.data[0].len());
    }

    #[test]
    fn test_coords() {
        let grid = Grid::new(DATA).unwrap();
        println!("{:?}", grid.data);

        assert_eq!(3, grid.value_at((0, 0)));
        assert_eq!(5, grid.value_at((1, 1)));
        assert_eq!(7, grid.value_at((3, 0)));
        assert_eq!(6, grid.value_at((0, 2)));
        assert_eq!(0, grid.value_at((4, 4)));
    }

    //The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom
    //since other trees of height 5 are in the way.)
    //The top-middle 5 is visible from the top and right.
    //The top-right 1 is not visible from any direction; for it to be visible, there would need
    //to only be trees of height 0 between it and an edge.
    //The left-middle 5 is visible, but only from the right.
    //The center 3 is not visible from any direction; for it to be visible, there would need to be
    //only trees of at most height 2 between it and an edge.
    //The right-middle 3 is visible from the right.
    //In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

    #[test]
    fn test_visibility() {
        let grid = Grid::new(DATA).unwrap();
        println!("{:?}", grid.data);
        assert_eq!(true, grid.visible((0, 0)));

        assert_eq!(5, grid.value_at((1, 1)));
        assert_eq!(5, grid.value_at((2, 1)));
        assert_eq!(1, grid.value_at((3, 1)));
        assert_eq!(5, grid.value_at((1, 2)));
        assert_eq!(3, grid.value_at((2, 2)));
        assert_eq!(3, grid.value_at((3, 2)));
        assert_eq!(3, grid.value_at((1, 3)));
        assert_eq!(5, grid.value_at((2, 3)));
        assert_eq!(4, grid.value_at((3, 3)));

        assert_eq!(true, grid.visible((1, 1)), "(1, 1)");
        assert_eq!(true, grid.visible((2, 1)), "(2, 1)");
        assert_eq!(false, grid.visible((3, 1)), "(3, 1)");
        assert_eq!(true, grid.visible((1, 2)), "(1, 2)");
        assert_eq!(false, grid.visible((2, 2)), "(2, 2)");
        assert_eq!(true, grid.visible((3, 2)), "(3, 2)");
        assert_eq!(false, grid.visible((1, 3)), "(1, 3)");
        assert_eq!(true, grid.visible((2, 3)), "(2, 3)");
        assert_eq!(false, grid.visible((3, 3)), "(3, 3)");
    }
}
