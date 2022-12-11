use color_eyre::eyre::Result;
//use color_eyre::eyre::{eyre, Result};
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let program = parse_program(&input);
    let mut device = Device::new(program);

    let key_cycles: HashSet<u64> = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut signals: Vec<i64> = Vec::new();

    while device.cycle <= 220 {
        device.tick();
        if key_cycles.contains(&device.cycle) {
            let signal = device.cycle as i64 * device.x;
            println!("C:{}, X:{}, signal: {}", device.cycle, device.x, signal);
            signals.push(signal);
        }
    }

    println!("part1: {}", signals.iter().sum::<i64>());

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Code {
    NoOp,
    Addx(i64),
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Idle,
    Executing(usize, Code),
}

#[derive(Debug)]
struct Device {
    x: i64,
    cycle: u64,
    state: State,
    program: VecDeque<Code>,
}

impl Device {
    fn new(program: VecDeque<Code>) -> Device {
        Device {
            x: 1,
            cycle: 0,
            state: State::Idle,
            program,
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        self.state = match &self.state {
            State::Idle => match self.program.pop_front() {
                // duplicate logic in Executing branch. :/
                Some(Code::NoOp) => State::Executing(1, Code::NoOp),
                Some(Code::Addx(v)) => State::Executing(2, Code::Addx(v)),
                None => State::Idle,
            },
            State::Executing(counter, code) => {
                let counter = counter - 1;
                if counter == 0 {
                    match code {
                        Code::NoOp => (),
                        Code::Addx(v) => self.x += v,
                    }
                    match self.program.pop_front() {
                        Some(Code::NoOp) => State::Executing(1, Code::NoOp),
                        Some(Code::Addx(v)) => State::Executing(2, Code::Addx(v)),
                        None => State::Idle,
                    }
                } else {
                    State::Executing(counter, code.clone())
                }
            }
        }
    }
}

fn parse_program(data: &str) -> VecDeque<Code> {
    let mut program = VecDeque::new();
    for line in data.lines() {
        //println!("{line:?}");

        let code = match line.trim() {
            "noop" => Code::NoOp,
            code => match code.split_once(' ') {
                Some(("addx", x)) => Code::Addx(x.parse::<i64>().unwrap()),
                Some((code, _)) => unimplemented!("unrecognized opcode: {}", code),
                None => unreachable!(),
            },
        };
        program.push_back(code);
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let input: String = "noop\naddx 3\naddx -5".to_string();
        let program = parse_program(&input);
        let mut iter = program.into_iter();
        assert_eq!(Code::NoOp, iter.next().unwrap());
        assert_eq!(Code::Addx(3), iter.next().unwrap());
        assert_eq!(Code::Addx(-5), iter.next().unwrap());
    }

    // Execution of this program proceeds as follows:
    //
    // - At the start of the first cycle, the noop instruction begins execution. During the first
    // cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing
    // nothing.
    // - At the start of the second cycle, the addx 3 instruction begins execution. During
    // the second cycle, X is still 1.
    // - During the third cycle, X is still 1. After the third cycle, the addx 3 instruction
    // finishes execution, setting X to 4.
    // - At the start of the fourth cycle, the addx -5 instruction begins execution. During
    // the fourth cycle, X is still 4.
    // - During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction
    // finishes execution, setting X to -1.
    #[test]
    fn test_execute_test_program() {
        let input: String = "noop\naddx 3\naddx -5".to_string();
        let program = parse_program(&input);
        let mut device = Device::new(program);
        assert_eq!(State::Idle, device.state, "state");
        device.tick();
        assert_eq!(1, device.cycle, "cycle");
        assert_eq!(State::Executing(1, Code::NoOp), device.state, "state");
        assert_eq!(1, device.x, "X");
        device.tick();
        assert_eq!(2, device.cycle, "cycle");
        assert_eq!(State::Executing(2, Code::Addx(3)), device.state, "state");
        assert_eq!(1, device.x, "X");
        device.tick();
        assert_eq!(3, device.cycle, "cycle");
        assert_eq!(State::Executing(1, Code::Addx(3)), device.state, "state");
        assert_eq!(1, device.x, "X");
        device.tick();
        assert_eq!(4, device.cycle, "cycle");
        assert_eq!(State::Executing(2, Code::Addx(-5)), device.state, "state");
        assert_eq!(4, device.x, "X");
        device.tick();
        assert_eq!(5, device.cycle, "cycle");
        assert_eq!(State::Executing(1, Code::Addx(-5)), device.state, "state");
        assert_eq!(4, device.x, "X");
        device.tick();
        assert_eq!(6, device.cycle, "cycle");
        assert_eq!(State::Idle, device.state, "state");
        assert_eq!(-1, device.x, "X");
    }
}
