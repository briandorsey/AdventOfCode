use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut elves = vec![0];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += &line.parse::<u32>()?;
        }
        //println!("{line:?}");
    }

    elves.sort();
    elves.reverse();
    println!("day01p1: {:?}", &elves.first().unwrap());
    println!("day01p2: {:?}", &elves.iter().take(3).sum::<u32>());

    Ok(())
}
