use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::env;
//use std::fmt::Display;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;

    let mut r: HashMap<&str, &str> = HashMap::new();
    r.insert("one", "1");
    r.insert("two", "2");
    r.insert("three", "3");
    r.insert("four", "4");
    r.insert("five", "5");
    r.insert("six", "6");
    r.insert("seven", "7");
    r.insert("eight", "8");
    r.insert("nine", "9");
    print!("{:?}\n", r);

    let mut values = Vec::new();
    for line in _input.lines() {
        print!("{}\n", line);
        if let (Some(left), Some(right)) = (left(line, &r), right(line, &r)) {
            values.push(left * 10 + right);
        }
    }
    print!("{:?}\n", values);
    print!("output:\n{:?}\n", values.iter().sum::<u32>());

    Ok(())
}

fn left(s: &str, r: &HashMap<&str, &str>) -> Option<u32> {
    let nums: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    print!("{:?} -> ", nums);
    print!("first: {:?}\n", nums.first());
    nums.first().copied()
}

fn right(s: &str, r: &HashMap<&str, &str>) -> Option<u32> {
    let nums: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    print!("{:?} -> ", nums);
    print!("last: {:?}\n", nums.last());
    nums.last().copied()
}

fn _preprocess(s: &str) -> String {
    let mut out = s.replace("one", "1");
    out = out.replace("two", "2");
    out = out.replace("three", "3");
    out = out.replace("four", "4");
    out = out.replace("five", "5");
    out = out.replace("six", "6");
    out = out.replace("seven", "7");
    out = out.replace("eight", "8");
    out = out.replace("nine", "9");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
