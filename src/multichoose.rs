
#[derive(Debug, Clone)]
pub struct Multichoose {
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

impl Multichoose {
    pub fn new( dim: usize, n: usize ) -> Self {
        Multichoose {
            state: vec![ 0; dim ],
            status: Status::Ini,
            dim,
            n,
            end: vec![ n-1; dim ],
        }
    }
}


#[cfg(feature = "streaming")]
mod streaming_iterator {
    use super::{Multichoose, Status};
    use streaming_iterator::StreamingIterator;

    impl StreamingIterator for Multichoose {
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
                                }
                            }
                            for idx in 1..self.dim {
                                if self.state[idx] == self.n {
                                    self.state[idx] = self.state[idx-1];
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
    use super::{Multichoose, Status};
    use std::iter::Iterator;

    impl Iterator for Multichoose {
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
                                }
                            }
                            for idx in 1..self.dim {
                                if self.state[idx] == self.n {
                                    self.state[idx] = self.state[idx-1];
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
    fn detail() {
        let dim = 3;
        let n = 4;

        let mut iter = Multichoose::new( dim, n );
        
        assert_eq!( iter.next().unwrap(), &[0,0,0] );
        assert_eq!( iter.next().unwrap(), &[0,0,1] );
        assert_eq!( iter.next().unwrap(), &[0,0,2] );
        assert_eq!( iter.next().unwrap(), &[0,0,3] );
        assert_eq!( iter.next().unwrap(), &[0,1,1] );
        assert_eq!( iter.next().unwrap(), &[0,1,2] );
        assert_eq!( iter.next().unwrap(), &[0,1,3] );
        assert_eq!( iter.next().unwrap(), &[0,2,2] );
        assert_eq!( iter.next().unwrap(), &[0,2,3] );
        assert_eq!( iter.next().unwrap(), &[0,3,3] );
        assert_eq!( iter.next().unwrap(), &[1,1,1] );
        assert_eq!( iter.next().unwrap(), &[1,1,2] );
        assert_eq!( iter.next().unwrap(), &[1,1,3] );
        assert_eq!( iter.next().unwrap(), &[1,2,2] );
        assert_eq!( iter.next().unwrap(), &[1,2,3] );
        assert_eq!( iter.next().unwrap(), &[1,3,3] );
        assert_eq!( iter.next().unwrap(), &[2,2,2] );
        assert_eq!( iter.next().unwrap(), &[2,2,3] );
        assert_eq!( iter.next().unwrap(), &[2,3,3] );
        assert_eq!( iter.next().unwrap(), &[3,3,3] );
        assert_eq!( iter.next(), None );
    }
}
