use color_eyre::eyre::Result;
use std::default::Default;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut troop = Troop::new();

    let binding = input.lines().collect::<Vec<_>>();
    let segments = binding.split(|&e| e.is_empty()).collect::<Vec<_>>();
    for segment in segments {
        troop.monkeys.push(Monkey::parse(segment));
    }

    // simulate!
    for _ in 0..10_000 {
        troop.simulate_round();
    }

    println!("rounds: {}", troop.round);

    for (id, monkey) in troop.monkeys.iter().enumerate() {
        println!(
            "{id}: ins: {}  items: {}",
            monkey.inspections,
            monkey.items.len()
        );
    }

    let mut inspections: Vec<_> = troop.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();

    println!(
        "top inspections: {:?},  part 1: {}",
        &inspections[..2],
        &inspections[..2].iter().product::<u128>()
    );

    Ok(())
}

#[derive(Debug)]
enum Operation {
    OldAddOld,
    OldMultOld,
    OldAdd(u64),
    OldMult(u64),
    None,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Monkey {
    inspections: u128,
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(usize, u64)> {
        let mut output = Vec::new();
        // inspect & operate on worry levels.
        self.items.iter_mut().for_each(|i| match self.operation {
            Operation::OldAddOld => *i += *i,
            Operation::OldMultOld => *i *= *i,
            Operation::OldAdd(v) => *i += v,
            Operation::OldMult(v) => *i *= v,
            Operation::None => unimplemented!(),
        });

        // divide worry level (from part 1)
        //self.items.iter_mut().for_each(|i| *i /= 3);

        // throw items
        for item in self.items.drain(..) {
            self.inspections += 1;
            if item % self.test_divisor == 0 {
                output.push((self.test_true_monkey, item));
            } else {
                output.push((self.test_false_monkey, item));
            }
        }

        output
    }

    fn parse(spec: &[&str]) -> Monkey {
        let mut spec = spec.iter();
        // assume monkeys are in order
        spec.next();
        let items = match spec.next().expect("no items line").split_once(':') {
            Some((_, nums)) => nums
                .trim()
                .split(", ")
                .map(|e| e.parse::<u64>().expect("failed to parse item to int"))
                .collect::<Vec<_>>(),
            _ => unimplemented!(),
        };

        let operation = match spec
            .next()
            .expect("no operations line")
            .trim()
            .split_once("Operation: new = old ")
        {
            Some((_, op)) => match op.split_once(' ') {
                Some(("+", "old")) => Operation::OldAddOld,
                Some(("*", "old")) => Operation::OldMultOld,
                Some(("+", num)) => {
                    let num = num
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("parse int: '{}'", num));
                    Operation::OldAdd(num)
                }
                Some(("*", num)) => {
                    let num = num
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("parse int: '{}'", num));
                    Operation::OldMult(num)
                }
                Some((_, _)) => unimplemented!(),
                None => unreachable!(),
            },
            None => unreachable!(),
        };

        let (_, op) = spec
            .next()
            .expect("no test line")
            .trim()
            .split_once("Test: divisible by ")
            .expect("failed parsing test");
        let test_divisor = op.parse::<u64>().expect("failed parsing divisor");
        let test_true_monkey = spec
            .next()
            .expect("no true monkey")
            .trim()
            .split_once("If true: throw to monkey ")
            .expect("no true monkey text")
            .1
            .parse::<usize>()
            .expect("failed to parse true monkey index");
        let test_false_monkey = spec
            .next()
            .expect("no false monkey")
            .trim()
            .split_once("If false: throw to monkey ")
            .expect("no false monkey text")
            .1
            .parse::<usize>()
            .expect("failed to parse false monkey index");

        Monkey {
            items,
            operation,
            test_divisor,
            test_true_monkey,
            test_false_monkey,
            ..Monkey::default()
        }
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            inspections: 0,
            items: Vec::new(),
            operation: Operation::None,
            test_divisor: u64::MAX,
            test_true_monkey: usize::MAX,
            test_false_monkey: usize::MAX,
        }
    }
}

// consider making structs for Operations and Tests which implement Display to match the
// problem output.

struct Troop {
    monkeys: Vec<Monkey>,
    round: u128,
}

impl Troop {
    fn new() -> Troop {
        Troop {
            monkeys: Vec::new(),
            round: 0,
        }
    }

    fn simulate_round(&mut self) {
        self.round += 1;

        let mut troop_divisor = 1;
        // direct indexing seems likey a hackey work around... but direct
        // iteration runs into the borrow checker. I'm probably missing an
        // idiomatic pattern
        for id in 0..self.monkeys.len() {
            troop_divisor *= self.monkeys[id].test_divisor;

            let dispatch = self.monkeys[id].inspect();
            for (target, item) in dispatch {
                //println!("{id}: {target}, {item}");
                self.monkeys[target].items.push(item);
            }
        }

        for id in 0..self.monkeys.len() {
            self.monkeys[id]
                .items
                .iter_mut()
                .for_each(|i| *i %= troop_divisor);
        }
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
