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
/// the problem 4 gives '9009' as an example for a palindrome
fn is_palindrome_t00() {
    assert!(is_palindrome(9009));
}

#[test]
/// example given by the text as '2520' for numbers 1 to 10
fn problem_5_t00() {
    assert_eq!(2520, smallest_multiple(10));
}

#[test]
/// the problem gives for 10 the example 3025-385 = 2640
fn problem_6_t00() {
    assert_eq!(2640, sum_square_difference(10));
}

#[test]
/// the problem states the 6th prime as 13
fn problem_7_t00() {
    assert_eq!(13, nth_prime(6));
}

#[test]
/// example given by the problem for four digits
fn problem_8_t00() {
    assert_eq!((5832, vec!(9, 9, 8, 9)), largest_product_in_a_series(4));
}

#[test]
/// example given by the problem for n=10
fn problem_10_t00() {
    assert_eq!(17, summation_of_primes(10));
}

#[test]
/// example given by the problem for over five divisors
fn problem_12_t00() {
    assert_eq!(28, highly_divisible_triangle_number(5));
}

#[test]
/// example taken from problem 14
fn collatz_sequence_t00() {
    assert_eq!(vec!(13, 40, 20, 10, 5, 16, 8, 4, 2, 1), collatz_sequence(13));
}

#[test]
fn factorial_t00() {
    assert_eq!(1, factorial(0));
}

#[test]
fn factorial_t01() {
    assert_eq!(1, factorial(1));
}

#[test]
// example taken from problem 20
fn factorial_t02() {
    assert_eq!(3628800, factorial(10));
}

#[test]
// example given by problem 20
fn problem_20_t00() {
    assert_eq!(27, factorial_digit_sum(10));
}
