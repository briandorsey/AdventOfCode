use std::env;
use std::fs;

// this was an attempt to solve this one completely in one chain of functions... and it
// technically works, but I'm not happy with the readability of this solution.

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    // part 1
    let contains = input
        .lines()
        .map(|e| e.split(',').collect::<Vec<_>>())
        .map(|mut v| {
            v.iter_mut()
                .map(|e| {
                    e.split('-')
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|e| {
            //println!("{e:?}");
            let l = &e[0];
            let r = &e[1];
            {
                (l[0] <= r[0] && l[1] >= r[1]) || (r[0] <= l[0] && r[1] >= l[1])
            }
        })
        .filter(|e| *e)
        .count();
    println!("contains: {contains:?}");

    // part 2
    let overlaps = input
        .lines()
        .map(|e| e.split(',').collect::<Vec<_>>())
        .map(|mut v| {
            v.iter_mut()
                .map(|e| {
                    e.split('-')
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|e| {
            //print!("{e:?}");
            let l = &e[0];
            let r = &e[1];
            {
                // 0th point of l inside r
                (l[0] >= r[0] && l[0] <= r[1])
                    // 1st point of l inside r
                    || (l[1] >= r[0] && l[1] <= r[1])
                    // l overlaps r
                    || (l[0] <= r[0] && l[1] >= r[1])
                // r overlaps l covered by the first two
            }
        })
        //.inspect(|e| println!(" --> {e}"))
        .filter(|e| *e)
        .count();
    println!("overlaps: {overlaps:?}");

    Ok(())
}
