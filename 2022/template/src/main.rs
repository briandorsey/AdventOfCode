//use anyhow::bail;
use std::env;
use std::fs;

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;

    Ok(())
}
