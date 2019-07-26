
#[derive(Debug, Clone)]
pub struct Combination {
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
    End,
}

impl Combination {
    pub fn new( dim: usize, n: usize ) -> Self {
        Combination {
            state: (0..dim).collect(),
            status: Status::Ini,
            dim,
            n,
            end: (n-dim..n).collect(),
        }
    }
}


#[cfg(feature = "streaming")]
mod streaming_iterator {
    use super::{Combination, Status};
    use streaming_iterator::StreamingIterator;

    impl StreamingIterator for Combination {
        type Item = [usize];

        fn advance(&mut self) {
            match self.status {
                Status::Ini => { self.status = Status::Run; },
                Status::Run => {
                    if self.state == self.end {
                        self.status = Status::End;
                    } else {
                        // 1 を足す
                        self.state[self.dim-1] += 1;
                        
                        while {
                            // 繰り上げ処理
                            for idx in (1..self.dim).rev() {
                                if self.state[idx] == self.n {
                                    self.state[idx-1] += 1;
                                    self.state[idx] = 0;
                                }
                            }
                            for idx in 1..self.dim {
                                if self.state[idx] == 0 {
                                    self.state[idx] = self.state[idx-1] + 1;
                                }
                            }

                            self.state[self.dim-1] >= self.n
                        } {}
                    }
                },
                Status::End => {},
            }
        }

        fn get(&self) -> Option<&[usize]> {
            match self.status {
                Status::Run => Some(&self.state),
                _ => None,
            }
        }
    }
}



#[cfg(not(feature = "streaming"))]
mod iterator {
    use super::{Combination, Status};
    use std::iter::Iterator;

    impl Iterator for Combination {
        type Item = Vec<usize>;

        fn next(&mut self) -> Option<Vec<usize>> {
            match self.status {
                Status::Ini => { self.status = Status::Run; },
                Status::Run => {
                    if self.state == self.end {
                        self.status = Status::End;
                    } else {
                        // 1 を足す
                        self.state[self.dim-1] += 1;
                        
                        while {
                        
                        // 繰り上げ処理
                        for idx in (1..self.dim).rev() {
                            if self.state[idx] == self.n {
                                self.state[idx-1] += 1;
                                self.state[idx] = 0;
                            }
                        }
                        for idx in 1..self.dim {
                            if self.state[idx] == 0 {
                                self.state[idx] = self.state[idx-1] + 1;
                            }
                        }

                        self.state[self.dim-1] >= self.n
                    } {}
                    }
                },
                Status::End => {},
            }

            match self.status {
                Status::Run => Some(self.state.clone()),
                _ => None,
            }
        }
    }
}






#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn detail1() {
        let dim = 3;
        let n = 4;

        let mut iter = Combination::new( dim, n );

        assert_eq!( iter.next().unwrap(), &[0,1,2] );
        assert_eq!( iter.next().unwrap(), &[0,1,3] );
        assert_eq!( iter.next().unwrap(), &[0,2,3] );
        assert_eq!( iter.next().unwrap(), &[1,2,3] );
        assert_eq!( iter.next(), None );
    }
    
    
    #[test]
    fn detail2() {
        let dim = 3;
        let n = 5;
        
        let mut iter = Combination::new( dim, n );
        
        assert_eq!( iter.next().unwrap(), &[0,1,2] );
        assert_eq!( iter.next().unwrap(), &[0,1,3] );
        assert_eq!( iter.next().unwrap(), &[0,1,4] );
        assert_eq!( iter.next().unwrap(), &[0,2,3] );
        assert_eq!( iter.next().unwrap(), &[0,2,4] );
        assert_eq!( iter.next().unwrap(), &[0,3,4] );
        assert_eq!( iter.next().unwrap(), &[1,2,3] );
        assert_eq!( iter.next().unwrap(), &[1,2,4] );
        assert_eq!( iter.next().unwrap(), &[1,3,4] );
        assert_eq!( iter.next().unwrap(), &[2,3,4] );
        assert_eq!( iter.next(), None );
    }
}
