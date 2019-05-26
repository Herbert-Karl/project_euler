//! module containing problem 10

use crate::common::is_prime;

/// problem 10
///
/// sum of all primes lower than N
pub fn summation_of_primes(n: u128) -> u128 {
    (1..n).into_iter().filter(|&x| is_prime(x)).sum()
}
