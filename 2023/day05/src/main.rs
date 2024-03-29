use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::cmp::min;
use std::env;
use std::fs;
use std::ops::Range;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let (seeds, path) = parse(input)?;
    //println!("{seeds:?}\n{path:?}");

    // part01
    println!("part01:");
    println!("seed count: {:?}", seeds.len());
    let min_locations = lookup_min_locations(&seeds, &path);

    //println!("min_locations: {min_locations:?}");
    println!("min_locations count: {:?}", min_locations.len());
    println!(
        "part01: min_location: {:?}",
        min_locations.iter().min().unwrap()
    );

    // part02
    println!("part02:");
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for item in seeds.chunks_exact(2) {
        let seed = item[0].clone();
        let count = item[1].clone();
        ranges.push((seed, seed + count));
    }
    ranges.sort();
    println!("ranges: {:?}", ranges.len());

    let mut totals: Vec<usize> = Vec::new();
    for r in &ranges {
        totals.push(r.1 - r.0);
    }
    println!("totals: {:?}", totals);
    println!("total: {:?}", totals.iter().sum::<usize>());

    let ranges = chunk_ranges(ranges, 10_000_000);

    let min_locations: Vec<usize> = ranges
        .par_iter()
        .map(|r| range_lookup_min_location(r.0..r.1, &path))
        .collect();

    //println!("min_locations: {min_locations:?}");
    println!("min_locations count: {:?}", min_locations.len());
    println!(
        "part02: min_location: {:?}",
        min_locations.iter().min().unwrap()
    );
    // part02 took 269.69s on a release build on my machine. Pretty slow, but still brute forced.
    //  258.99s user 0.87s system 99% cpu 4:20.41 total
    // Probably need to re-work the problem by caching paths to make things quicker?
    // also, this would be a good use-case to play with Rayon for parallelization while it's
    // still slow. .
    // rayon release, spread by range (uses less and less proc as some complete early)
    // 609.76s user 0.95s system 515% cpu 1:58.55 total
    // chunking the ranges in to 10_000_000 size chunks make it slower?
    // 1249.74s user 2.25s system 944% cpu 2:12.60 total
    // hmmm... maybe build is different than run? faster this time:
    // 516.41s user 2.16s system 873% cpu 59.379 total
    // huh... or maybe it just varies a lot?
    // 1097.76s user 2.43s system 945% cpu 1:56.33 total
    // Good enough for now

    Ok(())
}

fn chunk_ranges(ranges: Vec<(usize, usize)>, size: usize) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    for r in ranges {
        let total = r.1 - r.0;
        let whole = total / size;
        //let remainder = total % size;
        for i in 0..whole {
            let offset = i * size;
            output.push((r.0 + offset, r.0 + ((i + 1) * size)));
        }
        output.push((r.0 + whole * size, r.1));
    }
    output
}

fn lookup_min_locations(seeds: &Vec<usize>, path: &Vec<CatMap>) -> Vec<usize> {
    let mut min_locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut seed_path: Vec<usize> = vec![seed.clone()];
        for map in path {
            seed_path.push(map.map_forward(seed_path.last().unwrap().clone()))
        }
        //println!("{seed_path:?}");
        min_locations.push(seed_path.last().unwrap().clone());
    }
    println!("paths count: {:?}", min_locations.len());
    min_locations
}

fn range_lookup_min_location(seeds: Range<usize>, path: &Vec<CatMap>) -> usize {
    println!(
        "range_lookup..({seeds:?}, ...) total: {:?}",
        seeds.end - seeds.start
    );
    let mut min_location: usize = usize::MAX;
    for seed in seeds {
        let mut seed_path: Vec<usize> = vec![seed.clone()];
        for map in path {
            seed_path.push(map.map_forward(seed_path.last().unwrap().clone()))
        }
        //println!("{seed_path:?}");
        min_location = min(min_location, seed_path.last().unwrap().clone());
    }
    min_location
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

fn parse(input: String) -> Result<(Vec<usize>, Vec<CatMap>)> {
    // parse seeds
    let Some((seeds, input)) = input.split_once("\n") else {
        return Err(anyhow!("parsing: no newline to split on"));
    };
    // getting lazy, just using unwrap from here onward...
    let (_, seeds) = seeds.split_once(":").unwrap();
    let seeds: Vec<_> = seeds
        .trim()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

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
