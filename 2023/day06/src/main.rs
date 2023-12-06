use anyhow::Result;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut itr = input.lines();
    let time = itr.next().expect("should have a time entry");
    let (_, time) = time.split_once(":").expect("should have a :");
    let time = time.trim();
    let times: Vec<u64> = time
        .split_whitespace()
        .filter_map(|t| t.parse().ok())
        .collect();
    let part02_time = time.replace(" ", "").parse::<u64>().expect("parse u64");
    println!("{times:?}");

    let dist = itr.next().expect("should have a distance entry");
    let (_, dist) = dist.split_once(":").expect("should have a :");
    let dist = dist.trim();
    let distances: Vec<u64> = dist
        .split_whitespace()
        .filter_map(|t| t.parse().ok())
        .collect();
    let part02_distance = dist.replace(" ", "").parse::<u64>().expect("parse u64");
    println!("{distances:?}");

    println!("part01:");
    let mut wins: Vec<usize> = Vec::new();
    for race_idx in 0..times.len() {
        let time = times.get(race_idx).expect("time");
        let distance = distances.get(race_idx).expect("distance");
        println!(
            "Race {:?}: {:?}ms, record: {:?}mm",
            race_idx + 1,
            time,
            distance
        );
        wins.push(
            (0..=*time)
                .map(|t| traveled(&t, time))
                .filter(|d| d > distance)
                .count(),
        );

        println!("{wins:?}");
    }

    let mut margin = 1;
    for win in wins {
        margin *= win;
    }
    println!("part01: {:?}", margin);

    println!("\npart02:");
    println!("{part02_time:?}");
    println!("{part02_distance:?}");
    let wins = (0..=part02_time)
        .map(|t| traveled(&t, &part02_time))
        .filter(|d| d > &part02_distance)
        .count();

    println!("{wins:?}");

    Ok(())
}

fn traveled(time: &u64, allowed_time: &u64) -> u64 {
    let remaining = allowed_time - time;
    time * remaining
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
