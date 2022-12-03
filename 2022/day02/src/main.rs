use anyhow::bail;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        (match &self.response {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        } + match (&self.expected, &self.response) {
            // lose
            (Play::Scissors, Play::Paper)
            | (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock) => 0,
            // draw
            (&Play::Rock, &Play::Rock)
            | (&Play::Paper, &Play::Paper)
            | (&Play::Scissors, &Play::Scissors) => 3,
            // win
            (Play::Scissors, Play::Rock)
            | (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors) => 6,
        })
    }
}

fn parse_part_1(input: &str) -> anyhow::Result<Vec<Round>> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        if let Some((expected, response)) = line.trim().split_once(' ') {
            let expected = match expected {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                _ => bail!("error parsing '{line}' at '{expected}'"),
            };
            let response = match response {
                "X" => Play::Rock,
                "Y" => Play::Paper,
                "Z" => Play::Scissors,
                _ => bail!("error parsing '{line}' at '{response}'"),
            };
            let round = Round { expected, response };
            //println!("{line} --> {round:?}");
            rounds.push(round);
        }
    }
    Ok(rounds)
}

fn parse_part_2(input: &str) -> anyhow::Result<Vec<Round>> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        if let Some((expected, response)) = line.trim().split_once(' ') {
            let expected = match expected {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                _ => bail!("error parsing '{line}' at '{expected}'"),
            };
            let response = match response {
                // must lose
                "X" => match expected {
                    Play::Rock => Play::Scissors,
                    Play::Paper => Play::Rock,
                    Play::Scissors => Play::Paper,
                },
                // must draw
                "Y" => match expected {
                    Play::Rock => Play::Rock,
                    Play::Paper => Play::Paper,
                    Play::Scissors => Play::Scissors,
                },
                // must win
                "Z" => match expected {
                    Play::Rock => Play::Paper,
                    Play::Paper => Play::Scissors,
                    Play::Scissors => Play::Rock,
                },
                _ => bail!("error parsing '{line}' at '{response}'"),
            };
            let round = Round { expected, response };
            //println!("{line} --> {round:?}");
            rounds.push(round);
        }
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
        "day02p1: {:?}",
        rounds_1.iter().map(|e| e.score()).sum::<u32>()
    );
    println!(
        "day02p2: {:?}",
        rounds_2.iter().map(|e| e.score()).sum::<u32>()
    );

    Ok(())
}
