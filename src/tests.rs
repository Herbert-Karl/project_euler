//! module for tests

use super::*;

#[test]
/// the problem gives as text 'The prime factors of 13195 are 5, 7, 13 and 29.'
///
/// this example is used to test the problems solution
fn problem_3_t00() {
    assert_eq!(29, problem_0003::largest_prime_factor(13195));
}

#[test]
/// the problem 4 gives '9009' as an example for a palindrome
fn is_palindrome_t00() {
    assert!(common::is_palindrome(9009));
}

#[test]
/// example given by the text as '2520' for numbers 1 to 10
fn problem_5_t00() {
    assert_eq!(2520, problem_0005::smallest_multiple(10));
}

#[test]
/// the problem gives for 10 the example 3025-385 = 2640
fn problem_6_t00() {
    assert_eq!(2640, problem_0006::sum_square_difference(10));
}

#[test]
/// the problem states the 6th prime as 13
fn problem_7_t00() {
    assert_eq!(13, problem_0007::nth_prime(6));
}

#[test]
/// example given by the problem for four digits
fn problem_8_t00() {
    assert_eq!((5832, vec!(9, 9, 8, 9)), problem_0008::largest_product_in_a_series(4));
}

#[test]
/// example given by the problem for n=10
fn problem_10_t00() {
    assert_eq!(17, problem_0010::summation_of_primes(10));
}

#[test]
/// example given by the problem for over five divisors
fn problem_12_t00() {
    assert_eq!(28, problem_0012::highly_divisible_triangle_number(5));
}

#[test]
/// example taken from problem 14
fn collatz_sequence_t00() {
    assert_eq!(vec!(13, 40, 20, 10, 5, 16, 8, 4, 2, 1), common::collatz_sequence(13));
}

#[test]
fn factorial_t00() {
    assert_eq!(1, common::factorial(0));
}

#[test]
fn factorial_t01() {
    assert_eq!(1, common::factorial(1));
}

#[test]
/// example taken from problem 20
fn factorial_t02() {
    assert_eq!(3628800, common::factorial(10));
}

#[test]
/// example given by problem 20
fn problem_20_t00() {
    assert_eq!(27, problem_0020::factorial_digit_sum(10));
}

#[test]
/// example taken from problem 21
fn proper_divisors_t00() {
    let mut dividers = common::proper_divisors(220);
    dividers.sort();
    assert_eq!(vec!(1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110), dividers);
}

#[test]
/// example taken from problem 21
fn proper_divisors_t01() {
    let mut dividers = common::proper_divisors(284);
    dividers.sort();
    assert_eq!(vec!(1, 2, 4, 71, 142), dividers);
}

#[test]
/// example taken from problem 21
fn amicable_pair_t00() {
    assert_eq!(Some((220, 284)), common::amicable_pair(220));
}

#[test]
fn amicable_pair_t01() {
    assert_eq!(None, common::amicable_pair(1));
}

#[test]
/// example taken from problem 41
fn is_pandigital_t00() {
    assert!(common::is_pandigital(2143));
}

#[test]
/// example found while debugging a bug
fn is_pandigital_t01() {
    assert_eq!(false, common::is_pandigital(4441));
}
