# general
* switched to readign from stdin in the first version, but went back reading from a file later, since I didn't know how to (or if it's even *possible*) to re-process stdin. 

# confusion
* I still don't really have a good mental model of Error handling in main()
* looks like the compiler can get confused about match arms with guard conditions. Prevents simplifying some code or at least being sure all cases are dealt with. [unreachable!](https://doc.rust-lang.org/std/macro.unreachable.html)

# TIL
* match is lovely


# review and iterate
* Via: [JSMuellerRoemer](@JSMuellerRoemer@c.im) - [Source Code](https://github.com/l0calh05t/advent-of-code-2022/blob/trunk/src/solutions/day_02.rs) 
    * simplify line parsing with `.split_once()` and `if let` syntax. 
    * better error messages by including local `{variables}`
* Via: [Rosie Hamilton](@rosalita@mastodonapp.uk) - [Source Code](https://github.com/Rosalita/advent-of-code-2022/blob/main/rust/src/bin/2-1.rs) 
    * clarify the matches in `score()` by breaking out the logic for each rule
