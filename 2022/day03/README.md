
# confusion
* *really* struggled with trying to intersect three `HashSet<char>`s. It seems to create an iterator, and when you collect that, the values become references `&char`... and I couldn't figure out how to `.intersect()` with the final `HashSet<char>` because the interim one is a `HashSet<&char>`. my eventual work around was to pairwise intersect first&second and also second&third, storing each in a temp variable, and intersecting those, because the values match. Pretty sure I'm missing something. :) 

# TIL
* 

# review and iterate
* 

# things to learn about
* 

