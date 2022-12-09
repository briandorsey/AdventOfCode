Finally got a working solution, but it's neither efficient or idomatic. Clearly missing the right way to do things. 

## confusion / questions
* took a while to figure out how to get from a *probably* ASCII digit to an actual int. Ended up going via `char.to_digit()`
* `.take_while()` was *almost* perfect... needed one more element, something like `.take_until()`
* had more confusion/errors related to going between sequence types - slices and Vecs than I expected. Seems like I don't understand them as well as I thought. My implementations are much more complex than they should be, due to this. Might have simplified the solution to just double up the data representation, store both rows and cols. 

## TIL
* 

## review and iterate
* @spork@functional.cafe has a [solution](https://git.sr.ht/~mongus/advent2022/tree/main/item/src/day8.rs) implementing custom iterators (including a `StopAt` one which handles the `take_until` need). 
* @MichDdev@mastodon.social has a [solution](https://github.com/michd/advent-of-code/blob/main/2022/aoc08/src/main.rs) which calculates state ones and stores it in a struct per tree. Tests! 
* @sneeu@mastodon.social has a [solution](https://github.com/sneeu/advent-of-code-2022/blob/main/day08/src/main.rs) with separate data (`Vec<Vec<i32>>`) and functions. Idiom to generate column: `let column: Vec<i32> = forest.iter().map(|row| row[y]).collect();` Segments grabbed as slices.
* @weph@phpc.social has a [solution](https://philip-weinke.de/2022/12/advent-of-rust-8/) factored into DRY functions passing data along with no structs. `count()` helper function handles the `take_until` need.    (TODO: check back on GH project and change link to that once code is there)
* @fistons@fosstodon.org has a [solution](https://github.com/fistons/AOC-2022/blob/main/aoc_8/src/lib.rs) with direct imperative processing. No unwrap! :)  Segment parsing using `Vec.split()`, something to experiment with. Useful pattern of just calling `.next()` for separately handling values from short iterators like `.split()`
Mentioned this gem of an issue: [Inclusive version of `take_while`](https://github.com/rust-lang/rust/issues/62208).
* @mvgrim@hachyderm.io has a [solution](https://github.com/neoeinstein/aoc-2022/blob/main/src/bin/day08.rs) "...using the raw input string as the forest directly, and then index a position by keeping track of the gutter size (for the newline character(s))." Uses `DoubleEndedIterator`, and slices for segments. `.map_while()` to handle the `take_until` need.

## things to learn about
* `Vec.split()` details. Spend some time with it in Rust playground.


