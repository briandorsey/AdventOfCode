use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::ops::Range;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let (seeds_string, path) = parse(input)?;
    //println!("{seeds:?}\n{path:?}");

    // part01
    println!("part01:");
    let seeds = seeds_string
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    println!("{seeds:?}");
    let min_locations = lookup_min_locations(seeds, path);

    println!("min_locations: {min_locations:?}");
    println!(
        "part01: min_location: {:?}",
        min_locations.iter().min().unwrap()
    );
    Ok(())
}

fn lookup_min_locations(seeds: Vec<usize>, path: Vec<CatMap>) -> Vec<usize> {
    let mut min_locations: Vec<usize> = Vec::new();
    println!("paths:");
    for seed in seeds {
        let mut seed_path: Vec<usize> = vec![seed.clone()];
        for map in &path {
            seed_path.push(map.map_forward(seed_path.last().unwrap().clone()))
        }
        println!("{seed_path:?}");
        min_locations.push(seed_path.last().unwrap().clone());
    }
    min_locations
}

// map categories from one to another
#[allow(dead_code)]
#[derive(Debug)]
struct CatMap {
    name: String,
    source: String,
    dest: String,
    ranges: Vec<CatMapRange>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct CatMapRange {
    source: Range<usize>,
    dest: Range<usize>,
    length: usize,
}

impl CatMap {
    fn map_forward(&self, input: usize) -> usize {
        // default to original value
        let mut output = input.clone();
        for r in self.ranges.iter() {
            if r.source.contains(&input) {
                let offset = input - r.source.start;
                output = r.dest.start + offset;
                // stop checking after finding the first one
                break;
            }
        }
        output
    }
}

fn parse(input: String) -> Result<(Vec<String>, Vec<CatMap>)> {
    // parse seeds
    let Some((seeds, input)) = input.split_once("\n") else {
        return Err(anyhow!("parsing: no newline to split on"));
    };
    // getting lazy, just using unwrap from here onward...
    let (_, seeds) = seeds.split_once(":").unwrap();
    let seeds: Vec<_> = seeds.trim().split(" ").map(|s| s.to_string()).collect();

    // parse the maps
    let mut path: Vec<CatMap> = Vec::new();

    for map in input.split("\n\n") {
        let map = map.trim();
        //println!("{map:?}");
        let (name, range_str) = map.split_once(" map:\n").unwrap();
        let (source, dest) = name.split_once("-to-").unwrap();

        let mut ranges: Vec<CatMapRange> = Vec::new();
        for range in range_str.split("\n") {
            let mut items = range.split(" ");
            let dest_start = items.next().unwrap().parse::<usize>().unwrap();
            let source_start = items.next().unwrap().parse::<usize>().unwrap();
            let length = items.next().unwrap().parse::<usize>().unwrap();

            ranges.push(CatMapRange {
                source: source_start..source_start + length,
                dest: dest_start..dest_start + length,
                length: length,
            });
        }

        path.push(CatMap {
            name: name.to_string(),
            source: source.to_string(),
            dest: dest.to_string(),
            ranges: ranges,
        });
    }

    Ok((seeds, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
    #[test]
    fn test_seed_to_soil() {
        let m = CatMap {
            name: "seed-to-soil".to_string(),
            source: "seed".to_string(),
            dest: "soil".to_string(),
            ranges: vec![
                CatMapRange {
                    source: 98..98 + 2,
                    dest: 50..50 + 2,
                    length: 2,
                },
                CatMapRange {
                    source: 50..50 + 48,
                    dest: 52..52 + 48,
                    length: 48,
                },
            ],
        };
        //Seed number 79 corresponds to soil number 81.
        //Seed number 14 corresponds to soil number 14.
        //Seed number 55 corresponds to soil number 57.
        //Seed number 13 corresponds to soil number 13.
        //seed  soil
        //0     0
        //1     1
        //...   ...
        //48    48
        //49    49
        //50    52
        //51    53
        //...   ...
        //96    98
        //97    99
        //98    50
        //99    51
        let checks = vec![
            (79, 81),
            (14, 14),
            (55, 57),
            (13, 13),
            (0, 0),
            (1, 1),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (96, 98),
            (97, 99),
            (98, 50),
            (99, 51),
        ];
        for (i, o) in checks.iter() {
            println!(
                "map_forward({i:?}) -> {:?}, expected: {o:?}",
                m.map_forward(i.clone())
            );
            assert!(m.map_forward(i.clone()) == *o);
        }
    }
}
