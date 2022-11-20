use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, prelude::*, BufReader};

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

fn main() -> io::Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let mut report = Vec::<Entry>::new();

    // bit positions - which bit is more common? add for 1, sub for 0
    let mut bits = [0i32; 32];
    let mut max_bit_count = 0usize;
    //dbg!(bits);

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut bit_count = 0usize;
        let mut entry_data = Vec::<bool>::new();
        //println!("{line}");
        for (index, bit) in line.trim().chars().rev().enumerate() {
            bit_count += 1;
            print!("{index}:{bit} ");
            match bit {
                '0' => {
                    bits[index] -= 1;
                    entry_data.push(false);
                }
                '1' => {
                    bits[index] += 1;
                    entry_data.push(true);
                }
                _ => panic!("got something other than '0' or '1': {}", bit),
            }
        }
        println!("");
        if bit_count > max_bit_count {
            max_bit_count = bit_count
        };
        report.push(Entry(entry_data));
    }
    println!("{bits:?}");
    println!("{max_bit_count:?}");
    println!("{report:?}");

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    println!("gamma: {gamma:b}");
    for &i in bits[0..max_bit_count].iter().rev() {
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
