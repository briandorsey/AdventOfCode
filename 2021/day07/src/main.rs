use anyhow::Result;
use std::collections::HashMap;
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
        .with_max_level(Level::INFO)
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
    let initial: Vec<i64> = initial.split(',').map(|e| e.parse().unwrap()).collect();
    debug!("{initial:?}");
    debug!(
        "average: {}",
        initial.iter().sum::<i64>() as f64 / initial.len() as f64
    );

    let mut locations = HashMap::new();
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for e in initial {
        if e < min {
            min = e
        };
        if e > max {
            max = e
        };
        locations.entry(e).and_modify(|e| *e += 1).or_insert(1);
    }
    debug!("{locations:?}");
    debug!("min..max: {min}..{max}");

    let mut min_fuel = i64::MAX;
    for i in min..=max {
        let mut fuel_consumption = 0i64;
        for (loc, count) in &locations {
            let diff = (loc - i).abs();
            debug!(
                "({loc:>3}-{i:>3})*{count:>3}= ({:>3})*{count:>3} = {:>3}",
                diff,
                diff * count
            );
            fuel_consumption += diff * count;
        }
        debug!("--> {i}: {}", fuel_consumption);
        if fuel_consumption < min_fuel {
            min_fuel = fuel_consumption
        };
    }
    info!("minimum fuel usage: {min_fuel}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
