Continuing the trend, this one also is taking a very long time. Eventually got a part 1 working, and then the initial refactor for part 2 went surprisingly smoothly. But, I must be doing something slightly differently than the spec asks for, since it doesn't pass. Time for more logging and testing.  

Finally, finnally got it working. In the end it turned out to be a logic error from reading the diagonal section of the spec wrong. 

Did a lot of tracing and refactoring... so at least that practice is good for something? 


## confusion / questions
* still confused by string concatenation sometimes. Ended up using `something.push_str(&something_else)`. Is that idomatic?
* `usize` to `char`.... answer seems to be `char::from_digit(data as u32, 10)` ... really? ungh. 
* all of the self.segs[i] indexing is a mess, must be a better way. When I tried to use slices, I ran into ownership/borrow issues.


## TIL
* adding asserts to verify the exit state of movement was really helpful in development. Easy to see exactly where many logic errors happen.
* Reading is hard. Always read the actual words of the spec. "the tail always moves one step diagonally to keep up". 

## review and iterate
* has a [solution]()
* 

## things to learn about
* 

