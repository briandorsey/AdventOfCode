use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;
use std::env;
use std::fmt::Display;
use std::fs;
use std::ops::Deref;

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
        let outcome = is_orderd(packet, 1);
        println!("{outcome}");

        println!("");
    }
    println!("==============");

    let part1: usize = packets
        .iter()
        .map(|p| is_orderd(&p, 1))
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .filter(|(_, v)| *v)
        .map(|(i, _)| i)
        .sum();
    dbg!(part1);

    // 1048 low

    Ok(())
}

fn is_orderd((left, right): &(Item, Item), ind: usize) -> bool {
    /*
    println!("{:>ind$} Compare {} vs {}", "-", left, right);
    if left.len() == 0 {
        println!(
            "{:>ind$} Left side ran out of items, so inputs are in the right order",
            "-"
        );
        true
    } else if right.len() == 0 {
        println!(
            "{:>ind$} Right side ran out of items, so inputs are not in the right order",
            "-"
        );
        false
    } else {
    */
    match (left, right) {
        // todo rework logic to work from Items directly
        // there is actually no way to continue from here in this architecture... we need to move
        // onward, but we don't have the list anymore. initial logic needs to have a loop.
        (Item::Int(l), Item::Int(r)) if l == r => {
            println!("{:>ind$} Compare {} vs {}", "-", l, r);
            is_orderd(
                &(
                    Packet::new(left[1..].to_vec()),
                    Packet::new(right[1..].to_vec()),
                ),
                ind + 2,
            )
        }
        (Item::Int(l), Item::Int(r)) => {
            println!("{:>ind$} Compare {} vs {}", "-", l, r);
            l < r
        }
        (Item::Items(l), Item::Items(r)) => {
            if !is_orderd(&(Packet::new(l.to_vec()), Packet::new(r.to_vec())), ind + 2) {
                return false;
            }

            is_orderd(
                &(
                    Packet::new(left[1..].to_vec()),
                    Packet::new(right[1..].to_vec()),
                ),
                ind + 2,
            )
        }
        (Item::Int(l), Item::Items(r)) => {
            println!(
                "{:>ind$} Mixed types; convert left to [{}] and retry comparison",
                "-", l
            );
            is_orderd(
                &(
                    Packet::new([Item::Int(*l)].to_vec()),
                    Packet::new(r.to_vec()),
                ),
                ind + 2,
            )
        }
        (Item::Items(l), Item::Int(r)) => {
            println!(
                "{:>ind$} Mixed types; convert right to [{}] and retry comparison",
                "-", r
            );

            is_orderd(
                &(
                    Packet::new(l.to_vec()),
                    Packet::new([Item::Int(*r)].to_vec()),
                ),
                ind + 2,
            )
        }
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

struct Packet {
    p: Vec<Item>,
}

impl Packet {
    fn new(p: Vec<Item>) -> Packet {
        Packet { p }
    }
}

impl Deref for Packet {
    type Target = Vec<Item>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.p
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[{}]",
            self.p
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
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
