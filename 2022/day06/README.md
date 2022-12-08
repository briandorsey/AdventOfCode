Happy with how this one turned out overall. Yea! 🎉

## confusion / questions
* got stuck trying to call `.windows()` on a `Chars` iterator. Seems like the workaround is to create a `Vec` and call `.windows()` on that? Why doesn't `Chars` also implement windows()? Maybe things are moving towards the `.array_chunks()` nightly? 

## TIL
* `.windows()` remains awesome. 

## review and iterate
* rob@escaperooms.social has a [solution](https://github.com/rtsuk/advent_of_code_2022/blob/master/src/bin/day6.rs) which uses a generic implementation on a struct based scanner pattern. Also very simple test cases using a helper function. 
* MichDdev@mastodon.social has an imperative [solution](https://github.com/michd/advent-of-code/blob/main/2022/aoc06/src/main.rs) using `.chars().collect()` then  `.sort()` `.dedup()`.
* @madlep@hachyderm.io has a [solution](https://github.com/madlep/advent_of_code_2022_rust/blob/main/src/days/day06.rs) using `HashSet.insert()`'s return boolean to check if a `char` has been seen. 
* @kanathan@hachyderm.io has a [solution](https://github.com/kanathan/advent_code_2022/blob/master/src/day_6/main.rs) with an iteration chain approach. One helper function broken out, which nicely documents the process, making this very clean. 
* @JSMuellerRoemer@c.im has a[solution](https://github.com/l0calh05t/advent-of-code-2022/blob/trunk/src/solutions/day_06.rs) with an iteration chain attention to allocations and error handling. 
* @rosalita@mastodonapp.uk has a [solution](https://github.com/Rosalita/advent-of-code-2022/blob/main/rust/src/bin/6-1.rs) with an imperative approch. Clean helper function which doesn't need parameterization because it verfies the length stays the same. Just one character change between part 1 & 2.
* @simoncrowe@fosstodon.org has a very compact [solution](https://github.com/simoncrowe/aoc/blob/main/2022/06.rs) using `.windows(seq_len).enumerate().filter((...HashSet... == seq_len).map()`

* has a [solution]()


## things to learn about
* 

