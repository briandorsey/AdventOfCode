use anyhow::Result;
//use std::collections::HashMap;
use std::env;
use std::fmt;
//use std::fmt::Debug;
use std::fs;
use std::io::{prelude::*, BufReader};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
//use std::ops;

struct State(Vec<i32>);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut first = true;
        for elem in &self.0 {
            if !first {
                write!(f, ",")?;
            }
            write!(f, "{}", elem)?;
            first = false;
        }
        Ok(())
    }
}

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

    let mut state = State(initial);
    println!("Initial state: {}", state);
    for _ in 1..=80 {
        advance(&mut state);
        //println!( "After {:>2} day{}: {}", i, if i == 1 { " " } else { "s" }, state);
    }

    println!("total fish: {}", state.0.len());
    Ok(())
}

fn advance(state: &mut State) {
    let mut new_fish = 0;
    for elem in state.0.iter_mut() {
        *elem -= 1;
        // new fish!
        if *elem < 0 {
            *elem = 6;
            new_fish += 1;
        }
    }
    for _ in 0..new_fish {
        state.0.push(8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
