# enumcombinatorics

Enumerative Combinatorics for Rust.

Collection of permutation, combination, sequence and multichoose of `usize`s.

This crate provides iterators (or streaming iterators) generating all possible
permutations, combinations, ... of 0, 1, ..., n-1. Here is an example:

```
use enumcombinatorics::*;

let dim = 2;
let n = 3;

let mut iter = Sequence::new( dim, n );

assert_eq!( iter.next().unwrap(), &[0,0] );
assert_eq!( iter.next().unwrap(), &[0,1] );
assert_eq!( iter.next().unwrap(), &[0,2] );
assert_eq!( iter.next().unwrap(), &[1,0] );
assert_eq!( iter.next().unwrap(), &[1,1] );
assert_eq!( iter.next().unwrap(), &[1,2] );
assert_eq!( iter.next().unwrap(), &[2,0] );
assert_eq!( iter.next().unwrap(), &[2,1] );
assert_eq!( iter.next().unwrap(), &[2,2] );
assert_eq!( iter.next(), None );
```


## Providing structs

* `Permutation`: One element appears only once. Order has meaning.
* `Sequence`: One element appears many times. Order has meaning.
* `Combination`: One element appears only once. Order has no meaning.
* `Multichoose`: One element appears many times. Order has no meaning.


## Crate feature flags

* streaming
** Instead of `std::iter::Iterator`, use `streaming_iterator::StreamingIterator`
   via the `streaming_iterator` crate.
** This feature reduces the overhead for the `next()` method, because `Iterator` 
   trait requires cloning `Vec<usize>` every time.
** If this flag is enabled, then you can not use `for` loop due to the lack of
   `std::iter::Iterator`.




