use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_true() {
        assert!(true);
    }
}
