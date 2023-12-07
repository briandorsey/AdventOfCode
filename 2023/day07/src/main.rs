use anyhow::Result;
use itertools::Itertools;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;
    //println!("{input:?}");

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        //println!("{line}");
        let (cards, bid) = line.split_once(" ").expect("parse expected space");
        let bid: u32 = bid.parse().expect("parse bid expected int");
        let weights: Vec<u32> = cards
            .chars()
            .map(|c| match c {
                c if c.is_ascii_digit() => c.to_digit(10).expect("parse int"),
                'T' => 10,
                'J' => 0,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect();
        hands.push(Hand {
            hand_type: HandType::from_str(&cards).expect("error parsing HandType"),
            weights: weights,
            cards: cards.to_string(),
            bid: bid,
        })
    }
    hands.sort();
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        print!("{:?}: {hand:?} --> ", (idx + 1) as u32);
        let winning = (idx + 1) as u32 * hand.bid;
        println!("{:?} * {:?} = {winning:?}", (idx + 1) as u32, hand.bid);
        winnings += winning;
    }

    println!("part02: {winnings:?}");

    Ok(())
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    weights: Vec<u32>,
    cards: String,
    bid: u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct ParseHandTypeErr;

impl HandType {
    fn from_str(s: &str) -> Result<Self, ParseHandTypeErr> {
        if !s.len() == 5 {
            return Err(ParseHandTypeErr);
        };

        let mut grouped = Vec::new();
        for (_, group) in &s.chars().sorted().group_by(|e| *e) {
            grouped.push(group.collect::<Vec<_>>());
        }
        grouped.sort_by_key(|e| e.len());
        grouped.reverse();

        // promote jokers to largest group
        // this section is a hack, cleanup someday
        let mut jokers: Vec<char> = Vec::new();
        let mut joker_idx: usize = usize::MAX;
        for (idx, g) in grouped.iter_mut().enumerate() {
            if g[0] == 'J' && g.len() != 5 {
                jokers.extend(g.drain(..));
                joker_idx = idx;
                break;
            }
        }
        if joker_idx != usize::MAX {
            grouped.remove(joker_idx);
        }
        grouped[0].extend(jokers.drain(..));

        //println!("{:?}", grouped);
        if grouped[0].len() == 5 {
            assert_eq!(1, grouped.len());
            return Ok(HandType::FiveOfAKind);
        } else if grouped[0].len() == 4 {
            assert_eq!(2, grouped.len());
            Ok(HandType::FourOfAKind)
        } else if grouped[0].len() == 3 {
            if grouped[1].len() == 2 {
                assert_eq!(2, grouped.len());
                Ok(HandType::FullHouse)
            } else {
                assert_eq!(3, grouped.len());
                Ok(HandType::ThreeOfAKind)
            }
        } else if grouped[0].len() == 2 {
            if grouped[1].len() == 2 {
                assert_eq!(3, grouped.len());
                Ok(HandType::TwoPair)
            } else {
                assert_eq!(4, grouped.len());
                Ok(HandType::OnePair)
            }
        } else if grouped.len() == 5 {
            // all distinct
            Ok(HandType::HighCard)
        } else {
            unreachable!("{s:?}, {grouped:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HandType::*;

    #[test]
    fn test_true() {
        let mut v: Vec<HandType> = vec![
            FullHouse,
            FiveOfAKind,
            HighCard,
            ThreeOfAKind,
            TwoPair,
            FourOfAKind,
            OnePair,
        ];
        v.sort();

        assert_eq!(
            v,
            vec![
                HighCard,
                OnePair,
                TwoPair,
                ThreeOfAKind,
                FourOfAKind,
                FullHouse,
                FiveOfAKind,
            ]
        );
    }
}
