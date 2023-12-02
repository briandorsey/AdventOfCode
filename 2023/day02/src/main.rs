use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::env;
use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn new(id: u32, s: &str) -> Self {
        Game {
            id: id,
            pulls: Pull::vec_from_str(s),
        }
    }
}

#[derive(Debug)]
struct Pull {
    r: u32,
    g: u32,
    b: u32,
}

impl Pull {
    fn vec_from_str(s: &str) -> Vec<Self> {
        let mut pulls: Vec<Self> = Vec::new();
        for pull_str in s.split(";") {
            let pull_str = pull_str.trim();
            print!("    pull: {:?} --> ", pull_str);
            let mut pull = Pull { r: 0, g: 0, b: 0 };
            for group in pull_str.split(",") {
                if let Some((count, color)) = group.trim().split_once(" ") {
                    //print!("group: {count:?}, {color:?}\n");
                    if let Ok(count) = count.parse::<u32>() {
                        //print!("group: {count:?}, {color:?}\n");
                        match color {
                            "red" => pull.r = count,
                            "green" => pull.g = count,
                            "blue" => pull.b = count,
                            _ => continue,
                        }
                    } else {
                    };
                }
            }
            print!("{pull:?}\n");
            pulls.push(pull);
        }
        pulls
    }
}

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let _input = fs::read_to_string(input)?;
    let games = parse_input(_input);
    println!("{games:?}");

    let bag = Pull {
        r: 12,
        g: 13,
        b: 14,
    };

    let mut possible_ids: Vec<u32> = Vec::new();

    for game in games {
        if game
            .pulls
            .iter()
            .all(|p| (p.r <= bag.r && p.g <= bag.g && p.b <= bag.b))
        {
            possible_ids.push(game.id)
        }
    }
    println!(
        "{possible_ids:?} --> sum: {:?}\n",
        possible_ids.iter().sum::<u32>()
    );

    Ok(())
}

fn parse_input(input: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        print!("{:?}\n", line);
        let Some((g, pulls)) = line.split_once(":") else {
            print!("parsing: no ':' found");
            continue;
        };
        print!("{:?}\n", g);
        let Some(tid) = g.split_once(" ") else {
            print!("parsing: no ' ' found in game id");
            continue;
        };

        let Ok(id) = tid.1.parse::<u32>() else {
            print!("parsing: text game id to_digit failed");
            continue;
        };

        let game = Game::new(id, pulls);
        games.push(game);
    }
    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
