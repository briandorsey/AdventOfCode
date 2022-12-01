use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let filepath = env::args().nth(1).expect("please pass a file path");
    println!("parsing: {filepath}");
    let file = File::open(filepath)?;
    let buf = io::BufReader::new(file);

    let mut elves = vec![0];

    for line in buf.lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += str::parse::<u32>(&line)?;
        }
        //println!("{line:?}");
    }

    elves.sort();
    println!("day01p1: {:?}", &elves[elves.len() - 1]);
    println!(
        "day01p2: {:?}",
        &elves[elves.len() - 3..elves.len()].iter().sum::<u32>()
    );

    Ok(())
}
