use color_eyre::eyre::Result;
use tracing::{info, trace, Level};
use tracing_subscriber::FmtSubscriber;
//use color_eyre::eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

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
    let input = parse_input(&input);

    info!("part 1: {}", surface_area(&input));

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coords(i32, i32, i32);

fn surface_area(cubes: &HashSet<Coords>) -> u64 {
    let mut shape: HashMap<&Coords, u64> = HashMap::new();
    for cube in cubes {
        if shape.get_mut(&cube).is_some() {
            panic!("Coord already in shape: {:?}", cube);
        }
        trace!("C--{cube:?}");
        let mut exposed_sides = 6;
        let adjecents = vec![
            Coords(cube.0 + 1, cube.1, cube.2),
            Coords(cube.0 - 1, cube.1, cube.2),
            Coords(cube.0, cube.1 + 1, cube.2),
            Coords(cube.0, cube.1 - 1, cube.2),
            Coords(cube.0, cube.1, cube.2 + 1),
            Coords(cube.0, cube.1, cube.2 - 1),
        ];
        for side in adjecents {
            trace!("C--{cube:?}  T--{side:?}");
            if let Some(v) = shape.get_mut(&side) {
                trace!("  {side:?} v: {v}");
                *v -= 1;
                exposed_sides -= 1;
            }
        }
        match shape.insert(cube, exposed_sides) {
            None => (),
            Some(coord) => panic!("Coord already in shape: {}", coord),
        }
    }
    shape.values().sum()
}

fn parse_input(input: &str) -> HashSet<Coords> {
    let mut output = HashSet::new();
    for line in input.lines() {
        let data: Vec<i32> = line
            .split(',')
            .take(3)
            .map(|e| e.parse::<i32>().expect("parsing i32"))
            .collect();
        let (x, y, z) = (data[0], data[1], data[2]);
        output.insert(Coords(x, y, z));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_area_small() {
        let mut input: HashSet<Coords> = HashSet::new();
        input.extend([Coords(1, 1, 1), Coords(2, 1, 1)]);
        assert_eq!(10, surface_area(&input));
    }

    #[test]
    fn test_surface_area_medium() {
        let input = include_str!("../test.txt");
        let input = parse_input(input);
        assert_eq!(64, surface_area(&input));
    }
}
