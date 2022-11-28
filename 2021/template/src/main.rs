use anyhow::Result;
//use std::collections::VecDeque;
use std::env;
//use std::fmt;
//use std::fmt::Debug;
use std::fs;
use std::io::{prelude::*, BufReader};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
//use std::ops;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default subscriber failed");

    let input = env::args_os().nth(1).unwrap();
    info!("{input:?}");

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    debug!("START: parsing lines from input file");
    let initial = lines.next().expect("failed to read first line")?;
    let initial: Vec<i32> = initial.split(',').map(|e| e.parse().unwrap()).collect();
    println!("{initial:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
