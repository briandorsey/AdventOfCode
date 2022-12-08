
I tried to do this one as a chain of functions and only comparing ends of the ranges... and... I'm not happy with what I ended up with. :) 

## confusion
* I got somewhat lost in the logic for finding overlaps. My initial working solution was actually missing a case that I only noticed while adding comments later... and then realized that case was actually covered by previous logic. Case in point. Seems like using std::ops::Range itself and/or HashSets or something where the compilier could help with the logic would be better. Will keep an eye out for that when looking at other's solutions

## TIL
* `.inspect()` is neat. 

## review and iterate
* [@haskman@functional.cafe](https://github.com/ajnsit/aoc2022/blob/a5927d7082869c223446a547aad86729ccb05777/day4/src/main.rs#L34)
    * tuple -> Type conversion on return *just works*?! Nice. 
* [@JSMuellerRoemer@c.im](https://github.com/l0calh05t/advent-of-code-2022/blob/trunk/src/solutions/day_04.rs)
    * Nice use of `Range`. Initialization using `start..=end` format. And clear expression of the logic using `l.contains(r.start()) || l.contains(r.end()) || r.contains(l.start()) || r.contains(l.end());`
    * Detailed error handling. I don't think there are any `.unwraps()` or obvious `panic` potentials. 
* [@fil@hachyderm.io](https://gitlab.com/samoylovfp/aoc/-/blob/master/aoc2022/src/bin/day4.rs)
    * Clean tuple based solution. Like the documentation of logic via variable names.
* [@lewisdaleuk@dapchat.online](https://lewisdale.dev/post/advent-of-code-day-four/)
    * Blog writeup of parsing to `RangeInclusive`, populating `HashSet`s and very clean logic with `a.superset(&b) || b.superset(&a)` and `!a.is_disjoint(&b)` (I probably should have played with `HashSet` again, but was both a bit burned by it from day03 and optimizing pre-maturely: "What if we had really long ranges?"
* [@beeb@hachyderm.io](https://github.com/beeb/aoc-2022/blob/main/src/days/day04.rs)
    * [nom](https://docs.rs/nom/latest/nom/) based parsing. I need to look into this. It reads a bit complex at first look, but I need to learn some streaming binary parsing libary for future projects. Is this it? 
    * Struct based solution with methods. Very clear logic via good names. 
* [@xfix@fosstodon.org](https://github.com/xfix/advent-of-code-2022/blob/master/src/day4/mod.rs)
    * Minimal modeling, parse to four values and do the logic. Leads to a very compact implementation.
* [@Aedius@lavraievie.social](https://github.com/Aedius/aoc-2022/blob/main/day4/src/main.rs)
    * Compact solution using regex and a single pass calculating the data using a container holding updated incremental counts as data is processed. This also allows sharing the logic implementation between part1 & part2. 
* [@cawhitworth@mastodon.online](https://github.com/cawhitworth/aoc22/blob/main/day-4/src/main.rs)
    * Imperative solution example. 
* [@fistons@fosstodon.org](https://github.com/fistons/AOC-2022/blob/main/aoc_4/src/lib.rs)
    * `.collect_tuple()` 🤯
    * Helper functions and `Impl` traits lead to very readable main functions (`part1()`, `part2()`)
* [@kouhai@treehouse.systems](https://gist.github.com/kouhaidev/29b5244c98203fa2c59bc5fd4289db4d)
    * Custom iterator for the parsing! Strings in, arrays of a custom struct out. Seems like this would be very composable/reusable. 
* overall
    * Saw several examples of folks convering booleans to ints for counting. `accumulator + u64::from(bool)` Is this common in Rust? It feels... weird to me. 
    * Again... so much variety! Lots of ideas for improving my implementation. 


## things to learn about
* Need to get really comfy with handling Results & Options in iterator chains. 
* I also asked about learning iteration on Mastodon, and got [some recommendations](https://mastodon.gamedev.place/@ozkriff/109453788104107048) to read through.

