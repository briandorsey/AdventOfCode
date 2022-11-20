use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn main() -> io::Result<()> {
    let input = env::args_os().nth(1).unwrap();
    println!("{input:?}");

    let mut commands = Vec::<Command>::new();

    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut elements = line.split_whitespace();
        let cmd = elements.next().unwrap();
        let dist = elements.next().unwrap().parse::<u32>().unwrap();

        //println!("|{cmd:?}|{dist:?}");
        match cmd {
            "forward" => commands.push(Command::Forward(dist)),
            "down" => commands.push(Command::Down(dist)),
            "up" => commands.push(Command::Up(dist)),
            _ => continue,
        };
    }

    println!("{:?}", &commands[..5]);

    let mut position: u32 = 0;
    let mut aim: u32 = 0;
    let mut depth: u32 = 0;

    for command in commands {
        match command {
            Command::Forward(dist) => {
                position += dist;
                depth += aim * dist;
            }
            Command::Down(dist) => aim += dist,
            Command::Up(dist) => aim -= dist,
        }
        println!("{position}, {aim}, {depth}");
    }

    println!(
        "Ending position: {position}, depth: {depth} and total: {}",
        position * depth
    );

    Ok(())
}
