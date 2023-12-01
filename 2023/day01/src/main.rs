use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::env;
//use std::fmt::Display;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;

    let mut values = Vec::new();
    for line in _input.lines() {
        print!("{}\n", line);
        let nums: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        print!("{:?}\n", nums);
        print!("first: {:?}, last: {:?}\n", nums.first(), nums.last());
        if let (Some(first), Some(last)) = (nums.first(), nums.last()) {
            values.push(first * 10 + last);
        }
    }
    print!("{:?}\n", values);
    print!("output:\n{:?}\n", values.iter().sum::<u32>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
