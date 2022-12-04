//use anyhow::bail;
use std::env;
use std::fs;
//use std::ops::Range;

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let count = input
        .lines()
        .map(|e| e.split(',').collect::<Vec<_>>())
        .map(|mut v| {
            v.iter_mut()
                .map(|e| {
                    e.split("-")
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
                if (&l[0] <= &r[0] && &l[1] >= &r[1]) || (&r[0] <= &l[0] && &r[1] >= &l[1]) {
                    true
                } else {
                    false
                }
            }
        })
        .filter(|e| *e)
        .count();
    println!("{count:?}");

    //println!("{:?}", Range { start: 3, end: 5 });
    //for _i in (Range { start: 3, end: 8 }) {
    //println!("{i}");
    //}

    //println!("{:?}", (1..5).contains(&9));

    Ok(())
}
