use color_eyre::eyre::Result;
use itertools::{EitherOrBoth, Itertools};
use serde::Deserialize;
use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fs;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    let level = match env::args().nth(2) {
        Some(e) => match e.as_str() {
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        },
        None => Level::INFO,
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default subscriber failed");
    info!("{input:?}");
    let input = fs::read_to_string(input)?;

    let packets: Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut packets: Vec<_> = packets
        .iter()
        .map(|p| Item::Items(serde_json::from_str(p).expect("json")))
        .collect();
    let packet_pairs: Vec<_> = packets
        .chunks(2)
        .map(|p| (p[0].clone(), p[1].clone()))
        .collect();

    for (i, packet) in packet_pairs.iter().enumerate() {
        debug!("== Pair {} == ", i + 1);

        //debug!("- Compare {} vs. {}", packet.0, packet.1);
        let outcome = is_orderd(&packet.0, &packet.1, 1).unwrap();
        debug!("{outcome}");

        debug!("");
    }
    debug!("==============");

    let part1: usize = packet_pairs
        .iter()
        .map(|p| is_orderd(&p.0, &p.1, 1).unwrap())
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .filter(|(_, v)| *v)
        .map(|(i, _)| i)
        .sum();
    info!("part 1: {}", part1);

    let div_a = Item::Items(vec![Item::Items(vec![Item::Int(2)])]);
    let div_b = Item::Items(vec![Item::Items(vec![Item::Int(6)])]);
    packets.push(div_a.clone());
    packets.push(div_b.clone());
    packets.sort();
    let mut total = 1;
    for (i, p) in packets.iter().enumerate() {
        println!("{i:>3}: {p}");
        if p == &div_a || p == &div_b {
            total *= i + 1;
            println!("  ^--div");
        }
    }
    info!("part 2: {}", total);
    Ok(())
}

fn is_orderd(left: &Item, right: &Item, ind: usize) -> Option<bool> {
    match (left, right) {
        (Item::Int(l), Item::Int(r)) if l == r => {
            debug!("{:>ind$} Compare {} vs {}", "-", l, r);
            None
        }
        (Item::Int(l), Item::Int(r)) if l < r => {
            debug!("{:>ind$} Compare {} vs {}", "-", l, r);
            debug!(
                "  {:>ind$} Left side is smaller, so inputs are in the right order",
                "-"
            );
            Some(true)
        }
        (Item::Int(l), Item::Int(r)) if l > r => {
            debug!("{:>ind$} Compare {} vs {}", "-", l, r);
            debug!(
                "  {:>ind$} Right side is smaller, so inputs are not in the right order",
                "-"
            );
            Some(false)
        }
        (Item::Items(l), Item::Items(r)) => {
            debug!(
                "{:>ind$} Compare [{}] vs [{}]",
                "-",
                l.iter().join(","),
                r.iter().join(",")
            );
            for pair in l.iter().zip_longest(r.iter()) {
                let option = match pair {
                    EitherOrBoth::Both(l, r) => {
                        //debug!("{:>ind$} Compare {} vs {}", "-", l, r);
                        is_orderd(l, r, ind + 2)
                    }
                    EitherOrBoth::Right(_) => {
                        debug!("{:>ind$} Left side ran out of items, so inputs are not in the right order", "-");
                        Some(true)
                    }
                    EitherOrBoth::Left(_) => {
                        debug!("{:>ind$} Right side ran out of items, so inputs are not in the right order", "-");
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
            debug!(
                "{:>ind$} Mixed types; convert left to [{}] and retry comparison",
                "-", l
            );
            is_orderd(&Item::Items(vec![Item::Int(*l)]), right, ind + 2)
        }
        (Item::Items(_), Item::Int(r)) => {
            debug!(
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

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        };
        if is_orderd(self, other, 1).expect("is_order() failed!") {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("partial_cmp() failed!")
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Int(s), Item::Int(o)) => s == o,
            (Item::Int(_), Item::Items(_)) => false,
            (Item::Items(_), Item::Int(_)) => false,
            // is the below valid? I think so because eventually we get ints
            (Item::Items(s), Item::Items(o)) => s == o,
        }
    }
}

impl Eq for Item {}

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
