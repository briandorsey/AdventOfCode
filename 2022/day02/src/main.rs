use anyhow::bail;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    expected: Play,
    response: Play,
}

impl Round {
    fn score(&self) -> u32 {
        #[allow(clippy::identity_op)] // explicit + 0 for documenation
        match self {
            Round {
                expected: Play::Rock,
                response: Play::Rock,
            } => 1 + 3,
            Round {
                expected: Play::Paper,
                response: Play::Rock,
            } => 1 + 0,
            Round {
                expected: Play::Scissors,
                response: Play::Rock,
            } => 1 + 6,
            Round {
                expected: Play::Scissors,
                response: Play::Paper,
            } => 2 + 0,
            Round {
                expected: Play::Paper,
                response: Play::Paper,
            } => 2 + 3,
            Round {
                expected: Play::Rock,
                response: Play::Paper,
            } => 2 + 6,
            Round {
                expected: Play::Rock,
                response: Play::Scissors,
            } => 3 + 0,
            Round {
                expected: Play::Scissors,
                response: Play::Scissors,
            } => 3 + 3,
            Round {
                expected: Play::Paper,
                response: Play::Scissors,
            } => 3 + 6,
        }
    }
}

fn parse_part_1(input: &str) -> anyhow::Result<Vec<Round>> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut it = line.split(' ');
        let expected = match it.next() {
            Some("A") => Play::Rock,
            Some("B") => Play::Paper,
            Some("C") => Play::Scissors,
            Some(_) => bail!("error parsing"),
            None => bail!("error parsing"),
        };
        let response = match it.next() {
            Some("X") => Play::Rock,
            Some("Y") => Play::Paper,
            Some("Z") => Play::Scissors,
            Some(_) => bail!("error parsing"),
            None => bail!("error parsing"),
        };
        let round = Round { expected, response };
        //println!("{line} --> {round:?}");
        rounds.push(round);
    }
    Ok(rounds)
}

fn parse_part_2(input: &str) -> anyhow::Result<Vec<Round>> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut it = line.split(' ');
        let expected = match it.next() {
            Some("A") => Play::Rock,
            Some("B") => Play::Paper,
            Some("C") => Play::Scissors,
            Some(_) => bail!("error parsing"),
            None => bail!("error parsing"),
        };
        let response = match it.next() {
            // must lose
            Some("X") => match expected {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            // must draw
            Some("Y") => match expected {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissors => Play::Scissors,
            },
            // must win
            Some("Z") => match expected {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            Some(_) => bail!("error parsing"),
            None => bail!("error parsing"),
        };
        let round = Round { expected, response };
        //println!("{line} --> {round:?}");
        rounds.push(round);
    }
    Ok(rounds)
}

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");

    let input = fs::read_to_string(input)?;

    let rounds_1 = parse_part_1(&input)?;
    let rounds_2 = parse_part_2(&input)?;
    //dbg!(&rounds_1);

    println!(
        "day01p1: {:?}",
        rounds_1.iter().map(|e| e.score()).sum::<u32>()
    );
    println!(
        "day01p2: {:?}",
        rounds_2.iter().map(|e| e.score()).sum::<u32>()
    );

    Ok(())
}
