//! module for tests

use super::*;

#[test]
/// the problem gives as text 'The prime factors of 13195 are 5, 7, 13 and 29.'
///
/// this example is used to test the problems solution
fn problem_3_validation() {
    assert_eq!(29, largest_prime_factor(13195));
}
