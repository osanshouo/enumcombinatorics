//! 
//! 


#[derive(Debug, Clone)]
pub struct Permutation {
    state: Vec<usize>,
    status: Status,
    dim: usize,
    n: usize,
    end: Vec<usize>,
}

#[derive(Debug, Clone)]
enum Status {
    Ini,
    Run,
    End
}

impl Permutation {
    pub fn new( dim: usize, n: usize ) -> Permutation {
        Permutation {
            state: (0..dim).collect(),
            status: Status::Ini,
            dim, n,
            end: (n-dim..n).rev().collect(),
        }
    }

    fn has_duplication(&self) -> bool {
        let mut list: Vec<bool> = vec![ false; self.n ];
        for &i in self.state.iter() {
            if list[i] {
                return true;
            } else {
                list[i] = true;
            }
        }

        false
    }
}


#[cfg(feature = "streaming")]
mod streaming_iterator {
    use super::{Permutation, Status};
    use streaming_iterator::StreamingIterator;

    impl StreamingIterator for Permutation {
        type Item = [usize];

        fn advance(&mut self) {
            match self.status {
                Status::Ini => { self.status = Status::Run; },
                Status::Run => {
                    if self.state == self.end {
                        self.status = Status::End;
                    } else { while {
                        self.state[self.dim-1] += 1;
                        
                        for idx in (1..self.dim).rev() {
                            if self.state[idx] == self.n {
                                self.state[idx] = 0;
                                self.state[idx-1] += 1;
                            }
                        }
                        
                        self.has_duplication()
                    } {} }
                },
                Status::End => {},
            }
        }
        
        fn get(&self) -> Option<&[usize]> {
            match self.status {
                Status::Ini => None,
                Status::Run => Some(&self.state),
                Status::End => None,
            }
        }

        fn size_hint(&self) -> (usize,Option<usize>) {
            let size = (self.n-self.dim+1..self.n+1).fold(1, |prod, i| prod * i);
            ( 0, Some(size) )
        }
    }
}


#[cfg(not(feature = "streaming"))]
mod iterator {
    use super::{Permutation, Status};
    use std::iter::Iterator;
    
    impl Iterator for Permutation {
        type Item = Vec<usize>;
        
        fn next(&mut self) -> Option<Vec<usize>> {
            match self.status {
                Status::Ini => { self.status = Status::Run; },
                Status::Run => {
                    if self.state == self.end {
                        self.status = Status::End;
                    } else { while {
                        self.state[self.dim-1] += 1;
            
                        for idx in (1..self.dim).rev() {
                            if self.state[idx] == self.n {
                                self.state[idx] = 0;
                                self.state[idx-1] += 1;
                            }
                        }
                        
                        self.has_duplication()
                    } {} }
                },
                Status::End => {},
            }
            
            match self.status {
                Status::Ini => None,
                Status::Run => Some(self.state.clone()),
                Status::End => None,
            }
        }

        fn size_hint(&self) -> (usize,Option<usize>) {
            let size = (self.n-self.dim+1..self.n+1).fold(1, |prod, i| prod * i);
            ( 0, Some(size) )
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn permutation() {
        let dim = 2;
        let n = 8;

        let mut iter = Permutation::new( dim, n );
        let mut count = 0;

        while let Some(idx) = iter.next() {
            count += 1;
            println!("{:?}", idx);
        }

        assert_eq!( count, n * (n-1) );
    }


    #[test]
    fn datial() {
        let dim = 3;
        let n = 4;

        let mut iter = Permutation::new( dim, n );

        assert_eq!( iter.next().unwrap(), &[0,1,2] );
        assert_eq!( iter.next().unwrap(), &[0,1,3] );
        assert_eq!( iter.next().unwrap(), &[0,2,1] );
        assert_eq!( iter.next().unwrap(), &[0,2,3] );
        assert_eq!( iter.next().unwrap(), &[0,3,1] );
        assert_eq!( iter.next().unwrap(), &[0,3,2] );
        assert_eq!( iter.next().unwrap(), &[1,0,2] );
        assert_eq!( iter.next().unwrap(), &[1,0,3] );
        assert_eq!( iter.next().unwrap(), &[1,2,0] );
        assert_eq!( iter.next().unwrap(), &[1,2,3] );
        assert_eq!( iter.next().unwrap(), &[1,3,0] );
        assert_eq!( iter.next().unwrap(), &[1,3,2] );
        assert_eq!( iter.next().unwrap(), &[2,0,1] );
        assert_eq!( iter.next().unwrap(), &[2,0,3] );
        assert_eq!( iter.next().unwrap(), &[2,1,0] );
        assert_eq!( iter.next().unwrap(), &[2,1,3] );
        assert_eq!( iter.next().unwrap(), &[2,3,0] );
        assert_eq!( iter.next().unwrap(), &[2,3,1] );
        assert_eq!( iter.next().unwrap(), &[3,0,1] );
        assert_eq!( iter.next().unwrap(), &[3,0,2] );
        assert_eq!( iter.next().unwrap(), &[3,1,0] );
        assert_eq!( iter.next().unwrap(), &[3,1,2] );
        assert_eq!( iter.next().unwrap(), &[3,2,0] );
        assert_eq!( iter.next().unwrap(), &[3,2,1] );
        assert_eq!( iter.next(), None );
    }
}
    
