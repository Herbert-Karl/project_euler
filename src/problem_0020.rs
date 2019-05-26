//! module containing problem 20

use crate::common::factorial;

/// problem 20
///
/// finds the sum of the digits for N! (factorial)
pub fn factorial_digit_sum(n: u128) -> u128 {
    let fac = factorial(n);
    fac.to_string().chars().map(|x| x.to_digit(10).expect("this is no digit!") as u128).sum()
}
