use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let mut measurements = Vec::<u32>::new();

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        //let depth = u32::from_str(&line).unwrap();
        let depth: u32 = line
            .parse()
            .expect(&("error parsing to int: ".to_owned() + line.as_str()));
        measurements.push(depth);
    }
    println!("{} measurements", measurements.len());
    println!("{:?} ", &measurements[0..6]);

    let mut window_sums = Vec::<u32>::new();
    for w in measurements.windows(3) {
        window_sums.push(w.iter().sum());
    }
    println!("{} window_sums", window_sums.len());
    println!("{:?} ", &window_sums[0..4]);

    let mut count: u32 = 0;
    for w in window_sums.windows(2) {
        if w[0] < w[1] {
            count += 1
        }
    }
    println!("final count of increases: {count}");

    Ok(())
}
