//! module containing problem 2

use crate::common::fibonacci;

/// problem 2
///
/// sum of all even fibonacci numbers till N smaller then 4_000_000
pub fn even_fibonacci_numbers(n: u128) -> u128 {
    // because i dont know, till which fibonacci number, they are smaller 4 mil, i just create a big enough range to iterate over
    (0..n).into_iter().map(|x| fibonacci(x)).filter(|&x| x<4_000_000 && x%2==0).sum()
}
