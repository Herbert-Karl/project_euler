//! module containing problem 4

use crate::common::is_palindrome;

/// problem 4
///
/// largest palindrome product of two n-digit numbers
pub fn largest_palindrome_product(n: u32) -> u128 {
    let mut largest: u128 = 0;
    for i in 10_u128.pow(n-1)..10_u128.pow(n) {
        for j in 10_u128.pow(n-1)..10_u128.pow(n) {
            let product: u128 = i*j;
            if is_palindrome(product) && product>largest {
                largest = product;
            }
        }
    }
    largest
}
