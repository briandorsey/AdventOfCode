use color_eyre::eyre::Result;
use std::env;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;

    Ok(())
}
