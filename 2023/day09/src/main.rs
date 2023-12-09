use anyhow::Result;
//use anyhow::anyhow;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<i64>().expect("parsing ints"))
                .collect()
        })
        .collect();

    let mut h_finals: Vec<i64> = Vec::new();
    let mut h_first: Vec<i64> = Vec::new();
    for h in histories {
        println!("h: {h:?}");
        let mut finals: Vec<i64> = vec![h.last().expect("exists").clone()];
        let mut first: Vec<i64> = vec![h.first().expect("exists").clone()];
        let mut workspace: Vec<i64> = h.clone();
        loop {
            workspace = step_diff(&workspace);
            println!("{workspace:?}");
            finals.push(
                workspace
                    .last()
                    .expect(format!("ran out of items, while processing: {h:?}").as_str())
                    .clone(),
            );
            first.push(
                workspace
                    .first()
                    .expect(format!("ran out of items, while processing: {h:?}").as_str())
                    .clone(),
            );
            if workspace.iter().all(|e| *e == 0) {
                break;
            }
        }
        finals.reverse();
        h_finals.push(finals.into_iter().reduce(|acc, e| acc + e).expect("reduce"));
        first.reverse();
        h_first.push(first.into_iter().reduce(|acc, e| e - acc).expect("reduce"));
    }
    println!("part01: {:?}", h_finals);
    println!("part01: {:?}", h_finals.iter().sum::<i64>());

    println!("part02: {:?}", h_first);
    println!("part02: {:?}", h_first.iter().sum::<i64>());

    Ok(())
}

fn step_diff(input: &Vec<i64>) -> Vec<i64> {
    input.windows(2).map(|s| s[1] - s[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
