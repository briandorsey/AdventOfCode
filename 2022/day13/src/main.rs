use color_eyre::eyre::Result;
use itertools::{EitherOrBoth, Itertools};
use serde::Deserialize;
use std::env;
use std::fmt::Display;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let packets: Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let packets: Vec<_> = packets.chunks(2).map(|p| (p[0], p[1])).collect();
    let packets: Vec<(_, _)> = packets
        .iter()
        .map(|(l, r)| {
            (
                Item::Items(serde_json::from_str(l).expect("json")),
                Item::Items(serde_json::from_str(r).expect("json")),
            )
        })
        .collect();

    //println!("{:?}", is_orderd(&packets[0]));
    for (i, packet) in packets.iter().enumerate() {
        println!("== Pair {} == ", i + 1);

        //println!("- Compare {} vs. {}", packet.0, packet.1);
        let outcome = is_orderd(&packet.0, &packet.1, 1).unwrap();
        println!("{outcome}");

        println!();
    }
    println!("==============");

    let part1: usize = packets
        .iter()
        .map(|p| is_orderd(&p.0, &p.1, 1).unwrap())
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .filter(|(_, v)| *v)
        .map(|(i, _)| i)
        .sum();
    dbg!(part1);

    Ok(())
}

fn is_orderd(left: &Item, right: &Item, ind: usize) -> Option<bool> {
    match (left, right) {
        (Item::Int(l), Item::Int(r)) if l == r => {
            println!("{:>ind$} Compare {} vs {}", "-", l, r);
            None
        }
        (Item::Int(l), Item::Int(r)) if l < r => {
            println!("{:>ind$} Compare {} vs {}", "-", l, r);
            println!(
                "  {:>ind$} Left side is smaller, so inputs are in the right order",
                "-"
            );
            Some(true)
        }
        (Item::Int(l), Item::Int(r)) if l > r => {
            println!("{:>ind$} Compare {} vs {}", "-", l, r);
            println!(
                "  {:>ind$} Right side is smaller, so inputs are not in the right order",
                "-"
            );
            Some(false)
        }
        (Item::Items(l), Item::Items(r)) => {
            println!(
                "{:>ind$} Compare [{}] vs [{}]",
                "-",
                l.iter().join(","),
                r.iter().join(",")
            );
            for pair in l.iter().zip_longest(r.iter()) {
                let option = match pair {
                    EitherOrBoth::Both(l, r) => {
                        //println!("{:>ind$} Compare {} vs {}", "-", l, r);
                        is_orderd(l, r, ind + 2)
                    }
                    EitherOrBoth::Right(_) => {
                        println!("{:>ind$} Left side ran out of items, so inputs are not in the right order", "-");
                        Some(true)
                    }
                    EitherOrBoth::Left(_) => {
                        println!("{:>ind$} Right side ran out of items, so inputs are not in the right order", "-");
                        Some(false)
                    }
                };
                if option.is_none() {
                    continue;
                };
                return option;
            }
            None
        }
        (Item::Int(l), Item::Items(_)) => {
            println!(
                "{:>ind$} Mixed types; convert left to [{}] and retry comparison",
                "-", l
            );
            is_orderd(&Item::Items(vec![Item::Int(*l)]), right, ind + 2)
        }
        (Item::Items(_), Item::Int(r)) => {
            println!(
                "{:>ind$} Mixed types; convert right to [{}] and retry comparison",
                "-", r
            );
            is_orderd(left, &Item::Items(vec![Item::Int(*r)]), ind + 2)
        }
        (l, r) => todo!("{:?}, {:?}", l, r),
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Item {
    Int(i32),
    Items(Vec<Item>),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Item::Int(i) => write!(f, "{}", i)?,
            Item::Items(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
