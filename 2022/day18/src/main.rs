use color_eyre::eyre::Result;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use tracing::{info, trace, Level};
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
    let input = parse_input(&input);

    info!("part 1: {}", surface_area(&input));
    info!("part 2: {}", external_surface_area(&input));

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, PartialOrd, Ord)]
struct Coords(i32, i32, i32);

impl Display for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

fn build_shape(cubes: &HashSet<Coords>) -> HashMap<Coords, u64> {
    let mut shape: HashMap<Coords, u64> = HashMap::new();
    for cube in cubes {
        if shape.get_mut(cube).is_some() {
            panic!("Coord already in shape: {:?}", cube);
        }
        match insert_helper(&mut shape, *cube) {
            None => (),
            Some(_) => panic!("Coord already in shape: {}", cube),
        }
    }
    shape
}

fn insert_helper(shape: &mut HashMap<Coords, u64>, cube: Coords) -> Option<u64> {
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

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Bounds {
    fn new(shape: &HashMap<Coords, u64>) -> Bounds {
        let mut b = Bounds {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN,
        };
        for c in shape.keys() {
            if c.0 < b.min_x {
                b.min_x = c.0
            };
            if c.0 > b.max_x {
                b.max_x = c.0
            };
            if c.1 < b.min_y {
                b.min_y = c.1
            };
            if c.1 > b.max_y {
                b.max_y = c.1
            };
            if c.2 < b.min_z {
                b.min_z = c.2
            };
            if c.2 > b.max_z {
                b.max_z = c.2
            };
        }
        // pad each dimention with 1
        trace!("bounds: {:?}", b);
        b.min_x -= 1;
        b.max_x += 1;
        b.min_y -= 1;
        b.max_y += 1;
        b.min_z -= 1;
        b.max_z += 1;
        trace!("bounds: {:?}", b);
        b
    }

    fn contains(&self, c: &Coords) -> bool {
        c.0 >= self.min_x
            && c.0 <= self.max_x
            && c.1 >= self.min_y
            && c.1 <= self.max_y
            && c.2 >= self.min_z
            && c.2 <= self.max_z
    }
}

fn surface_area(cubes: &HashSet<Coords>) -> u64 {
    let shape = build_shape(cubes);
    shape.values().sum()
}

// oh hey, re-reading the question is useful. Let's try a "steam" fill method.
fn external_surface_area(cubes: &HashSet<Coords>) -> u64 {
    // determine bounds of the shape, add one to each dimention to find the cube to fill.
    // loop from a starting corner
    // * for 6 possible neighbors, lookup and see if a cube is there. if so, that's an
    // external cube, and +1 external face. If more air, then add to the list of steam locations
    // to check.
    // append current location to list of checked steam locations.
    //
    // Now we have a list of all steam locations, and have incremented for all external faces?
    // I think. Let's try it.

    trace!("-------- external_surface_area() -------");

    let shape = build_shape(cubes);
    let bounds = Bounds::new(&shape);

    // BTreeSet has pop methods, and regular HashSets don't
    let mut to_check: BTreeSet<Coords> = BTreeSet::new();
    let mut visited_steam: HashSet<Coords> = HashSet::new();
    let mut external_face_count = 0;

    // extra stuff for debugging help
    let mut found_at: Vec<Coords> = Vec::new();
    let mut found_at_set: HashSet<Coords> = HashSet::new();
    let mut found_count: HashMap<Coords, i32> = HashMap::new();

    // starting position should be outside of shape due to padding
    to_check.insert(Coords(bounds.min_x, bounds.min_y, bounds.min_z));
    loop {
        let Some(pos) = to_check.pop_last()  else{ break};
        trace!("check: {}", pos);
        assert!(!visited_steam.contains(&pos));
        assert!(!shape.contains_key(&pos));
        visited_steam.insert(pos);

        let mut adjecents = vec![
            Coords(pos.0 + 1, pos.1, pos.2),
            Coords(pos.0 - 1, pos.1, pos.2),
            Coords(pos.0, pos.1 + 1, pos.2),
            Coords(pos.0, pos.1 - 1, pos.2),
            Coords(pos.0, pos.1, pos.2 + 1),
            Coords(pos.0, pos.1, pos.2 - 1),
        ];
        for side in adjecents.drain(..) {
            if shape.get(&side).is_some() {
                trace!("   found shape at : {}", side);
                external_face_count += 1;
                found_at.push(pos);
                found_at_set.insert(pos);
                found_count.entry(pos).and_modify(|e| *e += 1).or_insert(1);
            } else if bounds.contains(&side) {
                if !visited_steam.contains(&side) {
                    trace!("   add to to_check: {:?}", side);
                    to_check.insert(side);
                } else {
                    trace!("   already checked: {}", side);
                }
            } else {
                trace!("   out of bounds  : {}", side);
            }
        }
    }

    trace!("to_check: {}  (expecting 0)", to_check.len());

    let x = bounds.max_x - bounds.min_x + 1;
    let y = bounds.max_y - bounds.min_y + 1;
    let z = bounds.max_z - bounds.min_z + 1;
    trace!("bounds: {} x {} x {} = {} ", x, y, z, x * y * z);
    trace!("visited_steam: {}", visited_steam.len());
    trace!("shape: {}", shape.len());
    trace!(
        " bounds - (visited_steam + shape) = {} spaces inside shape",
        (x * y * z) - (visited_steam.len() + shape.len()) as i32
    );
    trace!("found_at: {}", found_at.len());
    trace!("found_at_set: {}", found_at_set.len());
    //trace!("found_at_set: {:?}", found_at_set);
    trace!("found_count: {:?}", found_count);

    external_face_count
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
