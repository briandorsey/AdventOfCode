
## confusion / questions
* spent a long time debugging part02 for ordering issues ... I walked both secquences the wrong way... which was fine with addtion... but no subtraction. Bleh. 
Also having an extra 0 from the initialization state in .fold() was not problem for addition, but... does not give the same results with subtraction. :)

## TIL (Learned... or Looked up)
* ran into reference stuff I didn't understand when converting from fold() -> reduce(). Reduce seemed to really want to return a reference because I was iterating over a reference. But that was fine with fold? Ended up fighting the borrow checker for the first time in a while, before switching to iterating with into_iter() and consuming the sequence. Still don't understand why they're different when the docs look pretty much the same. 

## review and iterate
* 

## things to learn about
* 

