.... this took me .... forever. 

Coming back to it in the morning... I'm kinda happy with the overall architecture. This would be relatively easy to modify / re-use as needed. Seems like a trend for me ... things feel hard & take a long time, but overall very happy with the code that comes out in the end. 

## confusion / questions
* Tried for a very long time to use a data structure that probably isn't a good fit for Rust? A Vec of Dir notes representing the current working directory. Never did get it working that way. Fought with ownership stuff for a long time. 

## TIL
* I need to re-study references.

## review and iterate
* fasterthanlime ["In this article, we learn that building trees in #Rust the naive way is... painful."](https://fasterthanli.me/series/advent-of-code-2022/part-7). 
    * Another use of `nom`, let's consider that another +1 to look into it. 
    * There is a lot going on in this article. 
* @JSMuellerRoemer@c.im has a [solution](https://github.com/l0calh05t/advent-of-code-2022/blob/trunk/src/solutions/day_07.rs) with an `enum Node { File(usize), Directory(HashMap<String, Node>), }` as the core data structure & a recurisve `.size()` method. 
* @mvgrim@hachyderm.io has a [solution](https://github.com/neoeinstein/aoc-2022/blob/main/src/bin/day07.rs) using `petgraph::DiGraph` and `nom` parsing
* @Aedius@lavraievie.social has a [solution](https://github.com/Aedius/aoc-2022/blob/main/day7/src/main.rs) using `id_tree` and extensive helper library integration for the set of AoC solutions.

* has a [solution]()
* 

## things to learn about
* +1 to looking into `nom` parser crate
* `Rc` & `RefCell`
* struct destructuring in `match`
* using `|` in a `match` pattern
