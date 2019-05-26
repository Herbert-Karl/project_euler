//! module containing problem 14

use crate::common::collatz_sequence;

/// problem 14
///
/// computes number between 1 and N with the longest collatz sequence
pub fn longest_collatz_sequence(n: u64) -> u64 {
    let mut num = (1, 1);   // first number, second length of sequence
    for i in 1..n {
        let k = collatz_sequence(i);
        if k.len()>num.1 {
            num = (i, k.len());
        }
    }
    num.0
}
