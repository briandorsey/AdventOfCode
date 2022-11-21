use anyhow::Result;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{prelude::*, BufReader};

/// TOOD: find a better way to have simple errors in scripts. this is from
/// https://doc.rust-lang.org/std/error/trait.Error.html
#[derive(Debug)]
struct AnError;
impl fmt::Display for AnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error")
    }
}
impl Error for AnError {}

#[derive(Debug)]
struct Entry(Vec<bool>);

impl Entry {
    /// Create a new Entry from a string of 1s and 0s
    fn new(source: &str) -> Result<Self, AnError> {
        let mut data = Vec::<bool>::new();
        for bit in source.chars() {
            match bit {
                '0' => data.push(false),
                '1' => data.push(true),
                _ => return Err(AnError),
            };
        }
        Ok(Entry(data))
    }
}

fn main() -> Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let mut report = Vec::<Entry>::new();
    let mut max_bit_count = 0usize;

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let entry_data = Entry::new(line)?;
        if index < 10 {
            println!("{line}: {entry_data:?}");
        }
        report.push(entry_data);
    }

    // bit positions - which bit is more common? add for 1, sub for 0
    let mut bit_pos_counts = [0i32; 32];
    for entry in &report {
        if entry.0.len() > max_bit_count {
            max_bit_count = entry.0.len()
        }
        for (pos, val) in entry.0.iter().enumerate() {
            match val {
                true => bit_pos_counts[pos] += 1,
                false => bit_pos_counts[pos] -= 1,
            }
        }
    }
    println!("bit_pos_counts: {bit_pos_counts:?}");
    //println!( "report: {:?}", &report.iter().take(10).collect::<Vec<&Entry>>());

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    println!("gamma: {gamma:b}");
    for &i in bit_pos_counts[0..max_bit_count].iter() {
        gamma <<= 1;
        epsilon <<= 1;
        if i > 0 {
            gamma |= 0b00000001;
        } else {
            epsilon |= 0b00000001;
        }
        println!("{i}, gamma: {gamma:b}, epsilon: {epsilon:b}");
    }
    println!("gamma: {gamma:032b}, {gamma}, epsilon: {epsilon}");
    println!("power consumption: {}", gamma * epsilon);

    Ok(())
}
