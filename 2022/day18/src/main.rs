use color_eyre::eyre::Result;
use tracing::{debug, info, trace, Level};
use tracing_subscriber::FmtSubscriber;
//use color_eyre::eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter};
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
    info!("part 2: {}", external_surface_area(&input));
    // 2296 low

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Coords(i32, i32, i32);

impl Display for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

fn build_shape(cubes: &HashSet<Coords>) -> HashMap<Coords, u64> {
    let mut shape: HashMap<Coords, u64> = HashMap::new();
    for cube in cubes {
        if shape.get_mut(&cube).is_some() {
            panic!("Coord already in shape: {:?}", cube);
        }
        match insert_helper(&mut shape, *cube) {
            None => (),
            Some(_) => panic!("Coord already in shape: {}", cube),
        }
    }
    shape
}

fn insert_helper<'a>(shape: &mut HashMap<Coords, u64>, cube: Coords) -> Option<u64> {
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
        None => None,
        Some(_) => panic!("Coord already in shape: {}", cube),
    }
}

fn surface_area(cubes: &HashSet<Coords>) -> u64 {
    let shape = build_shape(cubes);
    shape.values().sum()
}

// deeeeply embarrased by the cut/paste of this approach, but I wanted to at least
// see if this logic worked. And then I needed to sleep. Checking it in just
// for reference of how bad this initial solution is.
// TODO: serious refactor.
// Nevermind, this doesn't work either. And it never would have. This tries to
// fill in external curves in the shape. Doh!
fn _external_surface_area_fill(cubes: &HashSet<Coords>) -> u64 {
    // try another approach: for each z & each y, check each x row ...
    // if any spots are missing, fill them in using the same alg as build_shape()
    let mut shape = build_shape(cubes);
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;
    for cube in shape.keys() {
        if cube.2 < min_z {
            min_z = cube.2
        }
        if cube.2 > max_z {
            max_z = cube.2
        }
    }

    let mut added_cubes: Vec<Coords> = Vec::new();

    for z in min_z..=max_z {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.2 == z { Some((c.0, c.1)) } else { None })
            .collect();
        level.sort();
        trace!("---- z:{z}: {level:?}");
        let mut x_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in level {
            x_map.entry(x).and_modify(|e| e.push(y)).or_insert(vec![y]);
        }
        for (x, y_vec) in x_map {
            trace!("---- z:{z}: x:{x} {y_vec:?}");
            let min = y_vec.iter().min().unwrap().clone();
            let max = y_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- z: {z} x:{x}, ys: {y_vec:?}");
                for y in min..=max {
                    if !y_vec.contains(&y) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (z,x): {}", added_cubes.len());

    for z in min_z..=max_z {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.2 == z { Some((c.0, c.1)) } else { None })
            .collect();
        level.sort();
        trace!("---- z:{z}: {level:?}");
        let mut y_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in level {
            y_map.entry(y).and_modify(|e| e.push(x)).or_insert(vec![x]);
        }
        for (y, x_vec) in y_map {
            trace!("---- z:{z}: x:{y} {x_vec:?}");
            let min = x_vec.iter().min().unwrap().clone();
            let max = x_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- z: {z} y:{y}, xs: {x_vec:?}");
                for x in min..=max {
                    if !x_vec.contains(&x) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (z,y): {}", added_cubes.len());

    // oh, right, we need to check for gaps along the z axis as well.
    // huge hack, but lets just... repeat the logic but swap x & z.
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    for cube in shape.keys() {
        if cube.0 < min_x {
            min_x = cube.0
        }
        if cube.0 > max_x {
            max_x = cube.0
        }
    }

    for x in min_x..=max_x {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.2 == x { Some((c.2, c.1)) } else { None })
            .collect();
        level.sort();
        trace!("---- x:{x}: {level:?}");
        let mut z_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut y_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (z, y) in level {
            z_map.entry(z).and_modify(|e| e.push(y)).or_insert(vec![y]);
            y_map.entry(y).and_modify(|e| e.push(z)).or_insert(vec![z]);
        }
        for (z, y_vec) in z_map {
            trace!("---- x:{x}: z:{z} {y_vec:?}");
            let min = y_vec.iter().min().unwrap().clone();
            let max = y_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- x: {x} z:{z}, ys: {y_vec:?}");
                for y in min..=max {
                    if !y_vec.contains(&y) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (x,z): {}", added_cubes.len());

    for x in min_x..=max_x {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.2 == x { Some((c.2, c.1)) } else { None })
            .collect();
        level.sort();
        trace!("---- x:{x}: {level:?}");
        let mut z_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut y_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (z, y) in level {
            z_map.entry(z).and_modify(|e| e.push(y)).or_insert(vec![y]);
            y_map.entry(y).and_modify(|e| e.push(z)).or_insert(vec![z]);
        }
        for (y, z_vec) in y_map {
            trace!("---- x:{x}: y:{y} {z_vec:?}");
            let min = z_vec.iter().min().unwrap().clone();
            let max = z_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- x: {x} y:{y}, zs: {z_vec:?}");
                for z in min..=max {
                    if !z_vec.contains(&z) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (x,y): {}", added_cubes.len());

    // .... and of course, y, too.
    // huge hack, but lets just... repeat the logic again????
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for cube in shape.keys() {
        if cube.1 < min_y {
            min_y = cube.1
        }
        if cube.1 > max_y {
            max_y = cube.1
        }
    }

    for y in min_y..=max_y {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.1 == y { Some((c.2, c.0)) } else { None })
            .collect();
        level.sort();
        trace!("---- y:{y}: {level:?}");
        let mut z_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (z, x) in level {
            z_map.entry(z).and_modify(|e| e.push(x)).or_insert(vec![x]);
        }
        for (z, x_vec) in z_map {
            trace!("---- y:{y}: z:{z} {x_vec:?}");
            let min = x_vec.iter().min().unwrap().clone();
            let max = x_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- y: {y} z:{z}, xs: {x_vec:?}");
                for x in min..=max {
                    if !x_vec.contains(&x) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (y,z): {}", added_cubes.len());

    for y in min_y..=max_y {
        let mut level: Vec<(i32, i32)> = shape
            .keys()
            .filter_map(|c| if c.1 == y { Some((c.2, c.0)) } else { None })
            .collect();
        level.sort();
        trace!("---- y:{y}: {level:?}");
        let mut x_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (z, x) in level {
            x_map.entry(x).and_modify(|e| e.push(z)).or_insert(vec![z]);
        }
        for (x, z_vec) in x_map {
            trace!("---- y:{y}: x:{x} {z_vec:?}");
            let min = z_vec.iter().min().unwrap().clone();
            let max = z_vec.iter().max().unwrap().clone();
            //println!("------- min: {} max: {}", min, max,);
            if (max - min) > 1 {
                debug!("-- y: {y} x:{x}, zs: {z_vec:?}");
                for z in min..=max {
                    if !z_vec.contains(&z) {
                        debug!("---- ({x}, {y}, {z}) missing {y}, adding.");
                        insert_helper(&mut shape, Coords(x, y, z));
                        added_cubes.push(Coords(x, y, z))
                    }
                }
            }
        }
    }
    info!("total added_cubes (y,x): {}", added_cubes.len());

    // double check that all the cubes we added are completely surrounded
    for cube in added_cubes {
        let Some(exposed_sides) = shape.get(&cube) else {
              panic!("just added cube wasn't in shape: {:?}", cube)
        };
        assert_eq!(
            0, *exposed_sides,
            "exposed sides: {exposed_sides} for {cube:?}",
        );
    }

    shape.values().sum()
}

// this doesn't work because projections can shadow texture
// had to plot the projections to understand, though
fn _external_surface_area_planes(cubes: &HashSet<Coords>) -> u64 {
    let shape = build_shape(cubes);
    let x_surface: HashSet<_> = shape.keys().map(|c| Coords(0, c.1, c.2)).collect();
    debug!("x_surface.len(): {}", x_surface.len());
    debug!(
        "x_surface: {}",
        x_surface
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let y_surface: HashSet<_> = shape.keys().map(|c| Coords(c.0, 0, c.2)).collect();
    debug!("y_surface.len(): {}", y_surface.len());
    debug!(
        "y_surface: {}",
        y_surface
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let z_surface: HashSet<_> = shape.keys().map(|c| Coords(c.0, c.1, 0)).collect();
    debug!("z_surface.len(): {}", z_surface.len());
    debug!(
        "z_surface: {}",
        z_surface
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
    (x_surface.len() * 2 + y_surface.len() * 2 + z_surface.len() * 2)
        .try_into()
        .unwrap()
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

    #[test]
    fn test_surface_area_input() {
        let input = include_str!("../input.txt");
        let input = parse_input(input);
        assert_eq!(4512, surface_area(&input));
    }

    #[test]
    fn test_external_surface_area_medium() {
        let input = include_str!("../test.txt");
        let input = parse_input(input);
        assert_eq!(58, external_surface_area(&input));
    }
}
