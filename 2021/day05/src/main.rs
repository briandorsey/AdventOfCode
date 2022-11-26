use anyhow::Result;
use std::collections::HashMap;
use std::env;
//use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::io::{prelude::*, BufReader};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
//use std::ops;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line(Point, Point);

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

    let mut vecs = Vec::<Line>::new();

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    debug!("START: parsing lines from input file");
    for line in lines {
        let line = line.unwrap();
        let mut raw_points = line.split(" -> ");
        let b = raw_points.next().unwrap();
        let e = raw_points.next().unwrap();
        //println!("{} --> {}", b, e);
        let mut raw_b = b.split(',');
        let beg = Point {
            x: raw_b.next().unwrap().parse().unwrap(),
            y: raw_b.next().unwrap().parse().unwrap(),
        };
        let mut raw_e = e.split(',');
        let end = Point {
            x: raw_e.next().unwrap().parse().unwrap(),
            y: raw_e.next().unwrap().parse().unwrap(),
        };

        debug!("{:?} --> {:?}", beg, end);
        vecs.push(Line(beg, end));
    }
    debug!("END: parsing lines from input file");

    // plot the coordinates affected by each vector
    let mut coords = HashMap::<Point, usize>::new();
    for v in vecs {
        let Line(a, b) = v;
        if a.x == b.x {
            info!("X: {a:?}, {b:?}");
            let min: usize;
            let max: usize;
            if a.y < b.y {
                min = a.y;
                max = b.y;
            } else {
                min = b.y;
                max = a.y;
            }

            for i in min..=max {
                let point = Point { x: a.x, y: i };
                debug!(" -> {point:?}");
                coords.entry(point).and_modify(|c| *c += 1).or_insert(1);
            }
        } else if a.y == b.y {
            info!("Y: {a:?}, {b:?}");
            let min: usize;
            let max: usize;
            if a.x < b.x {
                min = a.x;
                max = b.x;
            } else {
                min = b.x;
                max = a.x;
            }

            for i in min..=max {
                let point = Point { x: i, y: a.y };
                debug!(" -> {point:?}");
                coords.entry(point).and_modify(|c| *c += 1).or_insert(1);
            }
        } else {
            info!("skipping: {a:?}, {b:?}");
        }
    }

    //dbg!(coords);
    for (key, val) in coords.iter().filter(|(_, v)| **v >= 2) {
        println!("{key:?}: {val}");
    }
    println!(
        "total 2 or more: {}",
        coords.iter().filter(|(_, v)| **v >= 2).count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
