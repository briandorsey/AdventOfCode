use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
    let b: HashSet<_> = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect();
    let c: HashSet<_> = "PmmdzqPrVvPwwTWBwg".chars().collect();

    // this is what I expected to work based on understanding sets and reading that HashSet docs
    // (and seeing lots of chained function calls in Rust).
    //let in_all = a
    //    .intersection(&b)
    //    .collect::<HashSet<_>>()
    //    .intersection(&c)
    //    .collect::<HashSet<_>>();

    // This is where I ended up my own after fumbling around for longer than I'm willing to
    // admit
    let tmp1: HashSet<&char> = a.intersection(&b).collect();
    let tmp2: HashSet<&char> = b.intersection(&c).collect();
    let in_all: HashSet<&&char> = tmp1.intersection(&tmp2).collect();

    // looking at other solutions, lots of folks used .copied()... and just adding that almost
    // works... and the error message guides me to use annother let.
    //    let in_all = a
    //        .intersection(&b)
    //        .copied()
    //        .collect::<HashSet<_>>()
    //        .intersection(&c)
    //        .collect::<HashSet<_>>();

    // this works and is cleaner than my original solution - I'd be pretty happy to be guided here
    // from compiler error messages.
    //let in_all = a.intersection(&b).copied().collect::<HashSet<_>>();
    //let in_all = in_all.intersection(&c).collect::<HashSet<_>>();

    // however, a few folks online pointed out that HashSet implements `&` which is even cleaner
    // and nearly what I expected intersection to work like. It's still akward to give the extra
    // reference in order to make the third intersection work. But I imagine there is a nice way
    // to do this for the general case of a sequence of iteration. Maybe `.fold()` over an iterator or something?
    //let in_all = &(&a & &b) & &c;

    println!("{in_all:?}");
}
