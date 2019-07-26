//! Collection of permutation, combination, sequence and multichoose of `usize`s.
//! 
//! This crate provides iterators (or streaming iterators) generating all possible
//! permutations, combinations, ... of 0, 1, ..., n-1. Here is an example:
//! 
//! ```
//! use enumcombinatorics::*;
//! 
//! let dim = 2;
//! let n = 3;
//! 
//! let mut iter = Sequence::new( dim, n );
//! 
//! assert_eq!( iter.next().unwrap(), &[0,0] );
//! assert_eq!( iter.next().unwrap(), &[0,1] );
//! assert_eq!( iter.next().unwrap(), &[0,2] );
//! assert_eq!( iter.next().unwrap(), &[1,0] );
//! assert_eq!( iter.next().unwrap(), &[1,1] );
//! assert_eq!( iter.next().unwrap(), &[1,2] );
//! assert_eq!( iter.next().unwrap(), &[2,0] );
//! assert_eq!( iter.next().unwrap(), &[2,1] );
//! assert_eq!( iter.next().unwrap(), &[2,2] );
//! assert_eq!( iter.next(), None );
//! ```
//! 
//! 
//! # 
//! 
//! - `Permutation`: 1 number appears once in meaningful order
//! - `Sequence`: 1 number appears any times in meaningful order
//! - `Combination`: 1 number appears once in meaningless order
//! - `Multichoose`: 1 number appears any times in meaningless order
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! # Crate feature flags
//! 
//! - streaming
//! -- Instead of `std::iter::Iterator`, use `streaming_iterator::StreamingIterator`
//!    via the `streaming_iterator` crate.
//! -- This feature reduces the overhead for the `next()` method, because `Iterator` 
//!    trait requires cloning `Vec<usize>` every time.
//! -- If this flag is enabled, then you can not use `for` loop due to the lack of
//!    `std::iter::Iterator`.
//! 
//! 
//! 
//! 
//! 
//! 
//! 


#[cfg(feature = "streaming")]
pub use streaming_iterator::StreamingIterator;
#[cfg(not(feature = "streaming"))]
pub use std::iter::Iterator;


pub mod permutation;
pub use crate::permutation::Permutation;

pub mod sequence;
pub use crate::sequence::Sequence;

pub mod combination;
pub use combination::Combination;

pub mod multichoose;
pub use crate::multichoose::Multichoose;


