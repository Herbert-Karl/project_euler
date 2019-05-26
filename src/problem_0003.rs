//! module containing problem 3

use crate::common::{integer_sqrt, is_prime};

/// problem 3
///
/// largest prime factor of given number N
pub fn largest_prime_factor(n: u128) -> u128 {
    let mut list: Vec<u128> = (2..integer_sqrt(n)).into_iter().filter(|&x| n%x==0 && is_prime(x)).collect();
    list.pop().unwrap_or_else(|| {println!("No prime factors exist for {}! Returning zero.", n); 0})
}
