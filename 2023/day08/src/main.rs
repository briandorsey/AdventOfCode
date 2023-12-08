use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut itr = input.lines();

    let path = itr.next().expect("no lines in input");
    println!("{path:?}");
    let _ = itr.next(); // skip blank line

    for line in itr {
        let (key, line) = line.split_once(" = (").expect("parse: splitting on '='");
        let line = line.replace(")", "");
        let (l, r) = line.split_once(", ").expect("parse: splitting on ',')");
        node_map.insert(
            key.to_string(),
            Node {
                l: l.to_string(),
                r: r.to_string(),
            },
        );
    }
    //println!("{node_map:?}");

    let limit: u64 = 10_000_000_000;
    let mut counter = 0;
    let mut current: Vec<String> = node_map
        .keys()
        .filter(|k| k.chars().last().expect("key") == 'A')
        .map(|k| k.clone())
        .collect();
    //println!("{current:?}");
    for direction in path.chars().cycle() {
        counter += 1;
        if counter > limit {
            println!("limit reached. {limit:?} iterations. Quitting.");
            break;
        }

        for element in current.iter_mut() {
            //println!("{direction:?}");
            let node = node_map
                .get(element)
                .expect("node lookup failed: {current:?}");
            *element = match direction {
                'L' => node.l.clone(),
                'R' => node.r.clone(),
                _ => unreachable!(),
            };
        }
        //println!("{current:?}");
        if current
            .iter()
            .all(|e| e.chars().last().expect("key") == 'Z')
        {
            println!("ZZZ found after {counter:?} iterations");
            break;
        };
    }

    Ok(())
}

#[derive(Debug)]
struct Node {
    l: String,
    r: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
