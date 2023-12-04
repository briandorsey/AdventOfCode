use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;
    println!("{input}");

    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let Some((_, data)) = line.split_once(':') else {
            continue;
        };
        let Some((winning, have)) = data.split_once('|') else {
            continue;
        };
        let winning = winning.trim();
        let have = have.trim();

        let mut winning: Vec<_> = winning
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();
        let have: Vec<_> = have
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        winning.sort_unstable();
        let mut wins: Vec<usize> = Vec::new();
        for num in &have {
            match winning.binary_search(&num) {
                Ok(_) => wins.push(num.clone()),
                Err(_) => (),
            }
        }

        let mut score: usize = 0;
        if wins.len() > 0 {
            score = 1;
            for _ in 0..wins.len() - 1 {
                score = score * 2;
            }
        }

        cards.push(Card {
            winning: winning,
            have: have,
            score: score,
            wins: wins,
        });
    }

    let mut total_score = 0;
    for (i, card) in cards.iter().enumerate() {
        println!("{:?}: {card:?}", i + 1);
        total_score += card.score;
    }
    println!("{total_score:?}");

    let mut card_counts = vec![1; cards.len()];
    println!("{card_counts:?}");
    for (i, card) in cards.iter().enumerate() {
        let current_count = card_counts[i];
        //println!("i: {i:?}, cc: {card_counts:?}, w: {:?}", card.wins.len());
        if card.wins.len() > 0 {
            for j in i + 1..i + 1 + card.wins.len() {
                //println!("{j:?}");
                *card_counts.get_mut(j).unwrap() += current_count;
            }
        }
        println!("c: {:?}, w: {:?}: {card_counts:?}", i + 1, card.wins.len());
    }

    println!(
        "{card_counts:?} -> sum: {:?}",
        card_counts.iter().sum::<usize>()
    );

    Ok(())
}

#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    have: Vec<usize>,
    score: usize,
    wins: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
