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
    x: i32,
    y: i32,
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
        let min_y: i32;
        let max_y: i32;
        if a.y < b.y {
            min_y = a.y;
            max_y = b.y;
        } else {
            min_y = b.y;
            max_y = a.y;
        }
        let min_x: i32;
        let max_x: i32;
        if a.x < b.x {
            min_x = a.x;
            max_x = b.x;
        } else {
            min_x = b.x;
            max_x = a.x;
        }
        if a.x == b.x {
            info!("X: {a:?}, {b:?}");
            for i in min_y..=max_y {
                let point = Point { x: a.x, y: i };
                debug!(" -> {point:?}");
                coords.entry(point).and_modify(|c| *c += 1).or_insert(1);
            }
        } else if a.y == b.y {
            info!("Y: {a:?}, {b:?}");
            for i in min_x..=max_x {
                let point = Point { x: i, y: a.y };
                debug!(" -> {point:?}");
                coords.entry(point).and_modify(|c| *c += 1).or_insert(1);
            }
        } else {
            info!("diagonal: {a:?}, {b:?}");
            assert_eq!(max_x - min_x, max_y - min_y);
            let steps = max_x - min_x;
            let x_direction = if a.x < b.x { 1 } else { -1 };
            let y_direction = if a.y < b.y { 1 } else { -1 };
            for i in 0..=steps {
                let point = Point {
                    x: a.x + i * x_direction,
                    y: a.y + i * y_direction,
                };
                debug!(" -> {point:?}");
                coords.entry(point).and_modify(|c| *c += 1).or_insert(1);
            }
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
