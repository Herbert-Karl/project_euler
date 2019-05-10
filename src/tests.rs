//! module for tests

use super::*;

#[test]
/// the problem gives as text 'The prime factors of 13195 are 5, 7, 13 and 29.'
///
/// this example is used to test the problems solution
fn problem_3_t00() {
    assert_eq!(29, largest_prime_factor(13195));
}

#[test]
// the problem gives '9009' as an example for a palindrome
fn is_palindrome_t00() {
    assert!(is_palindrome(9009));
}

#[test]
fn problem_4_t00() {
    assert_eq!(2520, smallest_multiple(10));
}
