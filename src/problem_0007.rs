//! module containing problem 7

use crate::common::is_prime;

/// problem 7
///
/// calculates and returns the Nth prime number
pub fn nth_prime(n: u128) -> u128 {
    let mut x: (u128, u128) = (1, 2);
    while x.0<n {
        x.1 = x.1+1;
        if is_prime(x.1) {
            x.0 = x.0+1;
        }
    }
    x.1
}
