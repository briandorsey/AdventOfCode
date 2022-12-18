
## confusion / questions
* Why can't I sum u8s? ... or maybe... why is it a reference? All the values() call. ... let's try to dereference. Nope, it was u8 vs u64!
```
 1  error[E0277]: the trait bound `u64: Sum<&u8>` is not satisfied
     --> src/main.rs:23:5
      |
 23   |     shape.values().sum()
      |     ^^^^^^^^^^^^^^ --- required by a bound introduced by this call
      |     |
      |     the trait `Sum<&u8>` is not implemented for `u64`
```

## TIL (Learned... or Looked up)
* 

## review and iterate
* has a [solution]()
* 

## things to learn about
* 

