*phew* much more approachable than yesterday for me. Still easily thrown off by ... off by one errors. It's true what they say about the two hard problems in computer science. 

## confusion / questions
* Are methods with `&mut self` an antipattern? Seems like I can't call other methods on the struct in an `&mut self` method, so it's pretty limiting. But it also seems like we can't "upgrade" a an immutible borrow to a mutable one. Need to learn more about what kind of containers allow mutating contained values... seems like it works sometimes with `Vec`?  

## TIL
* 

## review and iterate
* @MauveAlert@tech.lgbt [posted](https://tech.lgbt/@MauveAlert/109492764601329890) clear examples of creating iterators. FWIW I used an existing iterator implementation 
[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e706064cb62bd63ad58beafcf5867b6c)
on nightly, you can use TAIT to hide the implementation detail, which is nice for more complicated types
[playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=4a5934b3a56ade6883df4db528a18f79)
    * their [solution](https://github.com/mauvealerts/advent-of-code/blob/main/aoc-2022/src/bin/day10.rs) uses iterator chains. It is very compact and still clear. No `unwrap` (outside of tests)!
        * `anyhow.ensure!` is neat!
        * `match` with `|`ed key cycles wouldn't have occured to me. 
* 

## things to learn about
* cases where we can get `&mut` of a portion of a container/struct/etc? 

