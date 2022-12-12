
## confusion / questions
* tried to store functions/closures on Monkey struct and couldn't figure it out. Seems like ... it's hard to store a closure on a struct? 
* part 2... things I tried:
    * just use u128, nope
    * switch to floats - got different inspection counts, so the throwing likely didn't work the same as before. Tried a bit of debugging with rounding the floats, etc, but didn't get it. 
    * switch to bigints? Found a crate and tried that... but... computation time is too long (likely by design, eh?)
    * did some searching and all of the divisors are ... suspiciously similar to numbers with divisibility rules. Primes, doh! Can likely work out the reduction rules for each divisor. ... but that seems like a lot of specialization for each divisor... and probably wouldn't work in the end because each monkey has a different divisor and they move around so it needs to work for all of them. 
    * ended up looking for hints on the math for part 2. 
* ended up doing... structs + methods style for the last few. For longer lived code, I think I like this pattern. But need to keep exercising the iteration chain muscles in Rust, right?

## TIL (Learned... or Looked up)
* looked up lots of things, but forgot to note them. :/
* when you specifically want side effects from an iterator, use `.for_each()` and to explicity force the iterator to be consumed, `.for_each(drop)`

## review and iterate
* 

## things to learn about
* how to implement an iterator

