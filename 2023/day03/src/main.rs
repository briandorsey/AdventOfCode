use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use grid::Grid;
use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;
    println!("{input}");

    let mut grid = Grid::<char>::new(0, 0);
    for line in input.lines() {
        grid.push_row(line.chars().collect())
    }

    let mut labels: HashMap<(usize, usize), String> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();

    for (y, row) in grid.iter_rows().enumerate() {
        let mut accumulator: String = String::new();
        for (x, item) in row.enumerate() {
            //println!("{x:?}, {y:?}, {item:?}");
            if item.is_digit(10) {
                accumulator.push(item.clone());
                if x + 1 == grid.cols() {
                    // duplication, ungh
                    labels.insert((x - (accumulator.len()), y), accumulator.clone());
                    accumulator.clear();
                }
            } else {
                if accumulator.len() > 0 {
                    //println!("{accumulator:?}");
                    labels.insert((x - (accumulator.len()), y), accumulator.clone());
                    accumulator.clear();
                };
                if *item != '.' {
                    symbols.insert((x, y), item.clone());
                }
            };
        }
    }
    println!("{symbols:?}");
    println!("{labels:?}");
    //println!("all labels.len(): {:?}", labels.values().len());
    //println!(
    //    "set labels.len(): {:?}",
    //    labels.values().collect::<HashSet<_>>().len()
    //);

    let mut part_nums: Vec<usize> = Vec::new();
    // gear location : label
    let mut gear_labels: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    'label: for ((x, y), label) in labels.iter() {
        print!("{x:?}, {y:?} {label:?}: ");
        //print!(
        //    "{:?}, {:?} ",
        //    x.saturating_sub(1)..x + 2 + label.len(),
        //    y.saturating_sub(1)..y.saturating_add(2)
        //);
        for yc in y.saturating_sub(1)..min(grid.rows(), y.saturating_add(2)) {
            for xc in x.saturating_sub(1)..min(grid.cols(), x + 1 + label.len()) {
                if let Some(symbol) = symbols.get(&(xc, yc)) {
                    if let Ok(num) = label.parse::<usize>() {
                        part_nums.push(num);
                        print!("part number! {:?} ", symbol);
                        //grid[(*y, *x)] = 'P';
                        if *symbol == '*' {
                            gear_labels
                                .entry((xc, yc))
                                .and_modify(|e| e.push(num))
                                .or_insert(vec![num]);
                            print!("inserted! {:?} ", symbol);
                        } else {
                            print!("skipped! {:?} ", symbol);
                        }
                    } else {
                        print!("failed to parse {:?} as digit", label);
                    }
                    println!();
                    continue 'label;
                }
                if grid[(yc, xc)] == '.' {
                    //grid[(yc, xc)] = '-';
                }
            }
        }
    }
    println!(
        "{part_nums:?} --> sum: {:?}",
        part_nums.iter().sum::<usize>()
    );
    println!(
        "{gear_labels:?} --> sum: {:?}",
        gear_labels
            .values()
            .filter(|g| g.len() == 2)
            .map(|g| g[0] * g[1])
            .sum::<usize>()
    );

    //    for row in grid.iter_rows() {
    //        for c in row {
    //            print!("{c}");
    //        }
    //        println!();
    //    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    fn grid_data() -> Grid<char> {
        let mut grid = Grid::<char>::new(0, 0);
        for line in DATA.lines() {
            grid.push_row(line.chars().collect())
        }
        grid
    }

    #[test]
    fn test_true() {
        let grid = grid_data();
        println!("{grid:?}");
        assert!(false);
    }
}
