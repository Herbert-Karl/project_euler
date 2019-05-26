//! module containing problem 21

use crate::common::amicable_pair;

/// problem 21
///
/// computes the sum of all amicable numbers under N
pub fn amicable_numbers(n: u128) -> u128 {
    let mut amicable_nums = Vec::new();
    for i in 1..n {
        if let Some(pair) = amicable_pair(i) {
            if pair.0<n && pair.1<n && !amicable_nums.contains(&pair.0) {
                amicable_nums.push(pair.0);
                amicable_nums.push(pair.1);
            }
        }
    }
    amicable_nums.into_iter().sum()
}
