
# confusion
* *really* struggled with trying to intersect three `HashSet<char>`s. It seems to create an iterator, and when you collect that, the values become references `&char`... and I couldn't figure out how to `.intersect()` with the final `HashSet<char>` because the interim one is a `HashSet<&char>`. my eventual work around was to pairwise intersect first&second and also second&third, storing each in a temp variable, and intersecting those, because the values match. Pretty sure I'm missing something. :) 

# TIL
* 

# review and iterate
* on the intersect question above: 
    * @queer_emma@mastodon.lol has a [solution](https://github.com/queer-emma/aoc2022/blob/main/src/day3.rs) which is pretty much what I was trying to implement, but never got right. Looks like `.copied()` is what I was missing.
    * []() dcreemer has a [solution](https://github.com/dcreemer/adventofcode/blob/main/2022/rust/three/src/main.rs) with a `.fold()` base implementation. 
    * []() pudnax has a [solution](https://github.com/pudnax/advent-of-code-2022/blob/master/src/solutions/day3.rs#L29) using `.find()` and `.contains()`
    * @JSMuellerRoemer@c.im has a [solution](https://github.com/l0calh05t/advent-of-code-2022/blob/trunk/src/solutions/day_03.rs) using ... some kind of binary math to manually compute the set. 
    * @xfix@fosstodon.org has a [solution](https://github.com/xfix/advent-of-code-2022/blob/master/src/day3/mod.rs) using `.retain()` and `.contains()`
    * @MichDdev@mastodon.social has a [solution](https://github.com/michd/advent-of-code/blob/main/2022/aoc03/src/main.rs) using a `.retain()` and `.any()`
    * @arch@floofy.tech has a [solution](https://git.sr.ht/~gmem/aoc-2022/tree/999966f52373fbbe1d125e968503553e6c8c78e5/item/03/src/main.rs) in one chain of function calls. 🤯
    * @kellan@fiasco.social has a [solution](https://github.com/kellan/aoc/blob/main/2022/rust/day3-rucksack/src/main.rs) with very clean functional calls by implementing helpers.
    * @gadiguibou@mastodon.online [points out that](https://mastodon.online/@gadiguibou/109450630889049793) "`std::HashSet` implements `&HashSet & &HashSet` with output HashSet as an intersection operator to solve exactly this." [docs link](https://doc.rust-lang.org/std/collections/struct.HashSet.html#impl-BitAnd%3C%26HashSet%3CT%2C%20S%3E%3E-for-%26HashSet%3CT%2C%20S%3E)
* @beeb@hachyderm.io has a [solution](https://github.com/beeb/aoc-2022/blob/main/src/days/day03.rs) which includes multiple implementations with timing, interesting to compare!
* @neofight78@mastodonapp.uk has a very consise [solution](https://github.com/neofight78/adventofcode2022/blob/master/day03/src/main.rs)

# things to learn about
* `.copied()`
* step through l0calh05t's solution to understand it
* https://lib.rs/crates/im - via [fasterthanlime](https://fasterthanli.me/series/advent-of-code-2022/part-3)

