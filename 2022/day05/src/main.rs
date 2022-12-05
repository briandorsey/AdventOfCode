use color_eyre::eyre::{eyre, Result};
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let (diagram, moves) = input
        .split_once("\n\n")
        .ok_or(eyre!("failed to split into diagram and moves"))?;
    println!("{diagram}");
    let mut stack_part1 = parse_diagram(diagram);
    let mut stack_part2 = parse_diagram(diagram);
    println!("-->");
    for (k, v) in &stack_part1 {
        println!("{k}: {v:?}");
    }
    println!();

    for line in moves.lines() {
        let action = parse_move(line);
        //println!("{action:?}");
        let (count, source, dest) = action;

        // process part1
        for _ in 0..count {
            let item1 = stack_part1
                .get_mut(&source)
                .expect("foo")
                .pop()
                .expect("failed to pop: {source}");
            stack_part1
                .get_mut(&dest)
                .expect("dest not found {dest}")
                .push(item1);
        }

        // process part2
        let mut buffer: Vec<char> = Vec::new();
        for _ in 0..count {
            let item = stack_part2
                .get_mut(&source)
                .expect("foo")
                .pop()
                .expect("failed to pop: {source}");
            buffer.push(item);
        }
        buffer.reverse();
        stack_part2
            .get_mut(&dest)
            .expect("dest not found {dest}")
            .extend_from_slice(&buffer);
        buffer.clear();
    }

    print!("Part1 top: ");
    for v in stack_part1.values() {
        print!("{}", v.last().unwrap());
    }
    println!();

    print!("Part2 top: ");
    for v in stack_part2.values() {
        print!("{}", v.last().unwrap());
    }
    println!();

    Ok(())
}

// return a BTreeMap where keys are stack names and values are a Vec of crates with the tail of
// the vec representing to top of the stack
fn parse_diagram(diag: &str) -> BTreeMap<char, Vec<char>> {
    let mut stacks: BTreeMap<char, Vec<char>> = BTreeMap::new();
    let mut iter = diag.lines().rev();
    //println!("{diag:?}");
    let index = match iter.next() {
        Some(s) => s,
        None => return stacks,
    };
    let mut indexes = index.chars().enumerate().collect::<Vec<_>>();
    indexes.retain(|(_, c)| c.is_numeric());
    //println!("{indexes:?}");
    for line in iter {
        let line = line.chars().collect::<Vec<char>>();
        //println!("{line:?}");
        for (i, name) in &indexes {
            //println!("{i} {name}");
            if !line[*i].is_whitespace() {
                stacks.entry(*name).or_default().push(line[*i]);
            }
        }
    }
    stacks
}

fn parse_move(line: &str) -> (u32, char, char) {
    let mut iter = line.split_whitespace();
    iter.next(); // consume "move"
    let count = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .expect("error parsing move");
    iter.next(); // consume "from"

    // there must be a cleaner way to do this
    let source = iter.next().unwrap().chars().next().unwrap();
    iter.next(); // consume "to"
    let dest = iter.next().unwrap().chars().next().unwrap();
    //println!("{count} {source} {dest}");
    (count, source, dest)
}

#[cfg(test)]
mod test {
    use super::*;

    const DIAGRAM: &str = include_str!("../test.txt");

    #[test]
    fn test_parse_diagram() {
        let (diagram, _) = DIAGRAM.split_once("\n\n").unwrap();
        let expected_keys = vec!['1', '2', '3'];

        let parsed = parse_diagram(diagram);
        let mut actual_keys = parsed.keys().copied().collect::<Vec<_>>();
        actual_keys.sort();
        assert_eq!(expected_keys, actual_keys);
        assert_eq!(&vec!['Z', 'N'], parsed.get(&'1').unwrap());
        assert_eq!(&vec!['M', 'C', 'D'], parsed.get(&'2').unwrap());
        assert_eq!(&vec!['P'], parsed.get(&'3').unwrap());
    }
}
