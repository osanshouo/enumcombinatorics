use enumcombinatorics::*;

/// This program prints the following:
/// [0, 1, 2]
/// [0, 1, 3]
/// [0, 2, 1]
/// [0, 2, 3]
/// [0, 3, 1]
/// [0, 3, 2]
/// [1, 0, 2]
/// [1, 0, 3]
/// [1, 2, 0]
/// [1, 2, 3]
/// [1, 3, 0]
/// [1, 3, 2]
/// [2, 0, 1]
/// [2, 0, 3]
/// [2, 1, 0]
/// [2, 1, 3]
/// [2, 3, 0]
/// [2, 3, 1]
/// [3, 0, 1]
/// [3, 0, 2]
/// [3, 1, 0]
/// [3, 1, 2]
/// [3, 2, 0]
/// [3, 2, 1]
/// 
fn main() {
    let dim = 3;
    let n = 4;

    let mut iter = Permutation::new( dim, n );

    while let Some(item) = iter.next() {
        println!("{:?}", item);
    }

}


