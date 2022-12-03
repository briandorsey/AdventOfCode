//use anyhow::bail;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Debug)]
struct Rucksack {
    a: HashSet<char>,
    b: HashSet<char>,
}

impl Rucksack {
    fn misfile(&self) -> char {
        let common: Vec<&char> = self.a.intersection(&self.b).collect();
        //println!("{common:?}");
        *common[0] // TODO: fix
    }
}

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut rucks: Vec<Rucksack> = Vec::new();
    for line in input.lines() {
        let contents = line.split_at(line.len() / 2);
        let ruck = Rucksack {
            a: contents.0.chars().collect(),
            b: contents.1.chars().collect(),
        };
        //println!("{contents:?}, {:?}, {:?}", ruck.a, ruck.b);
        rucks.push(ruck);
    }

    //println!( "{:?}", rucks.iter().map(|e| e.misfile()).collect::<Vec<char>>());

    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let priorities = ('a'..='z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .chain(('A'..='Z').enumerate().map(|(i, c)| (c, i + 27)))
        .collect::<HashMap<_, _>>();
    //println!("{priorities:?}");

    //println!( "{:?}", rucks .iter() .map(|e| e.misfile()) .map(|e| priorities.get(&e).unwrap()) .collect::<Vec<&usize>>());
    println!(
        "{:?}",
        rucks
            .iter()
            .map(|e| e.misfile())
            .map(|e| priorities.get(&e).unwrap())
            .sum::<usize>()
    );

    Ok(())
}
