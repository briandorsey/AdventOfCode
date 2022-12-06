use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;
    let part1 = find_marker(&input, 4).expect("couldn't find packet");
    println!("part1: {part1:?}");

    let part2 = find_marker(&input, 14).expect("couldn't find message");
    println!("part2: {part2:?}");
    Ok(())
}

fn find_marker(input: &str, window: usize) -> Option<usize> {
    let tmp: Vec<_> = input.chars().collect();
    for (i, w) in tmp.windows(window).enumerate() {
        let different = w.iter().collect::<HashSet<_>>().len() == window;
        //println!("{w:?}, {different}");
        if different {
            return Some(i + window);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    const TESTDATA: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn test_find_packet() {
        for (input, expected, _) in TESTDATA {
            println!("{input}, {expected}");
            assert_eq!(expected, find_marker(&input, 4).unwrap());
        }
    }
    #[test]
    fn test_find_message() {
        for (input, _, expected) in TESTDATA {
            println!("{input}, {expected}");
            assert_eq!(expected, find_marker(&input, 14).unwrap());
        }
    }
}
