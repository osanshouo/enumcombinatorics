
#[derive(Debug, Clone)]
pub struct Sequence {
    state: Vec<usize>,
    init: bool,
    dim: usize,
    n: usize,
}

impl Sequence {
    pub fn new( dim: usize, n: usize ) -> Sequence {
        Sequence { state: vec![ 0; dim], init: true, dim, n }
    }

    pub fn from_index( dim: usize, n: usize, idx: usize ) -> Option<Vec<usize>> {
        if idx >= n.pow(dim as u32) {
            None
        } else {
            let mut idx = idx;
            let mut vec = Vec::with_capacity(dim);
            
            for k in 0..dim {
                let x: usize = idx / n.pow((dim - 1 - k) as u32);
                vec.push( x );
                idx -= x * n.pow((dim - 1 - k) as u32);
            }

            Some(vec)
        }
    }
}


#[cfg(feature = "streaming")]
mod streaming_iterator {
    use super::Sequence;
    use streaming_iterator::StreamingIterator;

    impl StreamingIterator for Sequence {
        type Item = [usize];

        fn advance(&mut self) {
            if self.init {
                self.init = false;
            } else {
                self.state[self.dim-1] += 1;
            
                for idx in (1..self.dim).rev() {
                    if self.state[idx] == self.n {
                        self.state[idx] = 0;
                        self.state[idx-1] += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        fn get(&self) -> Option<&[usize]> {
            if self.state[0] >= self.n { None } else { Some(&self.state) }
        }
    }
}


#[cfg(not(feature = "streaming"))]
mod iterator {
    use super::Sequence;
    use std::iter::Iterator;

    impl Iterator for Sequence {
        type Item = Vec<usize>;

        fn next(&mut self) -> Option<Vec<usize>> {
            if self.init {
                self.init = false;
            } else {
                self.state[self.dim-1] += 1;
            
                for idx in (1..self.dim).rev() {
                    if self.state[idx] == self.n {
                        self.state[idx] = 0;
                        self.state[idx-1] += 1;
                    }
                }
            }

            if self.state[0] >= self.n {
                None
            } else {
                Some( self.state.clone() )
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn from_index() {
        let dim = 3;
        let n = 4;

        let mut iter = Sequence::new( dim, n );
        let mut idx = 0;

        while let Some(elem) = iter.next() {
            let vec = Sequence::from_index( dim, n, idx ).unwrap();
            let vec: &[usize] = &vec;
            assert_eq!( elem, vec );
            idx += 1;
        }

        assert_eq!( Sequence::from_index( dim, n, idx ), None );
    }
    
    
    #[test]
    fn detail() {
        let dim = 3;
        let n = 4;

        let mut iter = Sequence::new( dim, n );

        assert_eq!( iter.next().unwrap(), &[0,0,0] );
        assert_eq!( iter.next().unwrap(), &[0,0,1] );
        assert_eq!( iter.next().unwrap(), &[0,0,2] );
        assert_eq!( iter.next().unwrap(), &[0,0,3] );
        assert_eq!( iter.next().unwrap(), &[0,1,0] );
        assert_eq!( iter.next().unwrap(), &[0,1,1] );
        assert_eq!( iter.next().unwrap(), &[0,1,2] );
        assert_eq!( iter.next().unwrap(), &[0,1,3] );
        assert_eq!( iter.next().unwrap(), &[0,2,0] );
        assert_eq!( iter.next().unwrap(), &[0,2,1] );
        assert_eq!( iter.next().unwrap(), &[0,2,2] );
        assert_eq!( iter.next().unwrap(), &[0,2,3] );
        assert_eq!( iter.next().unwrap(), &[0,3,0] );
        assert_eq!( iter.next().unwrap(), &[0,3,1] );
        assert_eq!( iter.next().unwrap(), &[0,3,2] );
        assert_eq!( iter.next().unwrap(), &[0,3,3] );
        assert_eq!( iter.next().unwrap(), &[1,0,0] );
        assert_eq!( iter.next().unwrap(), &[1,0,1] );
        assert_eq!( iter.next().unwrap(), &[1,0,2] );
        assert_eq!( iter.next().unwrap(), &[1,0,3] );
        assert_eq!( iter.next().unwrap(), &[1,1,0] );
        assert_eq!( iter.next().unwrap(), &[1,1,1] );
        assert_eq!( iter.next().unwrap(), &[1,1,2] );
        assert_eq!( iter.next().unwrap(), &[1,1,3] );
        assert_eq!( iter.next().unwrap(), &[1,2,0] );
        assert_eq!( iter.next().unwrap(), &[1,2,1] );
        assert_eq!( iter.next().unwrap(), &[1,2,2] );
        assert_eq!( iter.next().unwrap(), &[1,2,3] );
        assert_eq!( iter.next().unwrap(), &[1,3,0] );
        assert_eq!( iter.next().unwrap(), &[1,3,1] );
        assert_eq!( iter.next().unwrap(), &[1,3,2] );
        assert_eq!( iter.next().unwrap(), &[1,3,3] );
        assert_eq!( iter.next().unwrap(), &[2,0,0] );
        assert_eq!( iter.next().unwrap(), &[2,0,1] );
        assert_eq!( iter.next().unwrap(), &[2,0,2] );
        assert_eq!( iter.next().unwrap(), &[2,0,3] );
        assert_eq!( iter.next().unwrap(), &[2,1,0] );
        assert_eq!( iter.next().unwrap(), &[2,1,1] );
        assert_eq!( iter.next().unwrap(), &[2,1,2] );
        assert_eq!( iter.next().unwrap(), &[2,1,3] );
        assert_eq!( iter.next().unwrap(), &[2,2,0] );
        assert_eq!( iter.next().unwrap(), &[2,2,1] );
        assert_eq!( iter.next().unwrap(), &[2,2,2] );
        assert_eq!( iter.next().unwrap(), &[2,2,3] );
        assert_eq!( iter.next().unwrap(), &[2,3,0] );
        assert_eq!( iter.next().unwrap(), &[2,3,1] );
        assert_eq!( iter.next().unwrap(), &[2,3,2] );
        assert_eq!( iter.next().unwrap(), &[2,3,3] );
        assert_eq!( iter.next().unwrap(), &[3,0,0] );
        assert_eq!( iter.next().unwrap(), &[3,0,1] );
        assert_eq!( iter.next().unwrap(), &[3,0,2] );
        assert_eq!( iter.next().unwrap(), &[3,0,3] );
        assert_eq!( iter.next().unwrap(), &[3,1,0] );
        assert_eq!( iter.next().unwrap(), &[3,1,1] );
        assert_eq!( iter.next().unwrap(), &[3,1,2] );
        assert_eq!( iter.next().unwrap(), &[3,1,3] );
        assert_eq!( iter.next().unwrap(), &[3,2,0] );
        assert_eq!( iter.next().unwrap(), &[3,2,1] );
        assert_eq!( iter.next().unwrap(), &[3,2,2] );
        assert_eq!( iter.next().unwrap(), &[3,2,3] );
        assert_eq!( iter.next().unwrap(), &[3,3,0] );
        assert_eq!( iter.next().unwrap(), &[3,3,1] );
        assert_eq!( iter.next().unwrap(), &[3,3,2] );
        assert_eq!( iter.next().unwrap(), &[3,3,3] );
        assert_eq!( iter.next(), None );
    }
}
    
