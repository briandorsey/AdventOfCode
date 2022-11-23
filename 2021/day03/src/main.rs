use anyhow::Result;
use std::cmp::max;
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

#[derive(Clone)]
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
    fn to_u32(&self) -> u32 {
        let mut value: u32 = 0;
        for &b in self.0.iter() {
            value <<= 1;
            if b {
                value |= 0b00000001;
            }
        }
        value
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Entry(")?;
        for v in &self.0 {
            let c = if *v { "1" } else { "0" };
            f.write_str(c)?
        }
        f.write_str(")")?;
        Ok(())
    }
}

type Report = Vec<Entry>;

fn calc_common_bit_values(report: &Report) -> Vec<i32> {
    // bit positions - which bit is more common? add for 1, sub for 0
    let max_bit_count = report.iter().fold(0, |m, e| max(m, e.0.len()));
    let mut bit_pos_counts: Vec<i32> = vec![0; max_bit_count];
    for entry in report {
        for (pos, val) in entry.0.iter().enumerate() {
            match val {
                true => bit_pos_counts[pos] += 1,
                false => bit_pos_counts[pos] -= 1,
            }
        }
    }
    bit_pos_counts
}

fn calculate_common(mut candidates: Report, bit_offset: usize, common: bool) -> Option<Entry> {
    println!("candidates.len(): {:?}", candidates.len());
    let bit_pos_counts = calc_common_bit_values(&candidates);

    if (bit_pos_counts[bit_offset] >= 0) ^ !common {
        candidates.retain(|e| e.0[bit_offset]);
    } else {
        candidates.retain(|e| !e.0[bit_offset]);
    }
    match candidates.len() {
        l if l < 1 => None,
        l if l > 1 => calculate_common(candidates, bit_offset + 1, common),
        _ => candidates.pop(),
    }
}

fn main() -> Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let mut report = Report::new();

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let entry_data = Entry::new(line)?;
        if index < 15 {
            println!("{line}: {entry_data:?}");
        }
        report.push(entry_data);
    }
    //println!( "report: {:?}", &report.iter().take(10).collect::<Vec<&Entry>>());

    // Compute power rating
    let bit_pos_counts = calc_common_bit_values(&report);
    println!("bit_pos_counts: {bit_pos_counts:?}");
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    println!("gamma: {gamma:b}");
    for &i in bit_pos_counts.iter() {
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

    // Compute life support rating
    let oxygen_candidates = report.clone();
    let oxygen = calculate_common(oxygen_candidates, 0, true).expect("failed to calculate oxygen");
    println!("oxygen: {oxygen:?}");
    let oxygen = oxygen.to_u32();
    println!("oxygen: {oxygen:?}, {oxygen:b}");

    // Compute CO2 scrubber rating
    let co2_candidates = report.clone();
    let co2 = calculate_common(co2_candidates, 0, false).expect("failed to calculate CO2");
    println!("co2: {co2:?}");
    let co2 = co2.to_u32();
    println!("co2: {co2:?}, {co2:b}");

    println!("life support rating: {}", oxygen * co2);

    Ok(())
}
