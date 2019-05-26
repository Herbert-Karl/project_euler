//! for solving the problems from project euler <https://projecteuler.net/>

mod common;
// problems
mod problem_0001;
mod problem_0002;
mod problem_0003;
mod problem_0004;
mod problem_0005;
mod problem_0006;
mod problem_0007;
mod problem_0008;
mod problem_0009;
mod problem_0010;
mod problem_0012;
mod problem_0014;
mod problem_0020;
mod problem_0021;
mod problem_0030;
#[cfg(test)]
mod tests;

fn main() {
    println!("Welcome to \"Solving Projecc Euler\"!");
    println!("Multiples of three and five for n = {}, is {}.", 1000, problem_0001::multiples_of_three_and_five(1000));
    println!("The sum of all even fibonacci numbers till the {}th smaller 4 mil, is {}.", 100, problem_0002::even_fibonacci_numbers(100));
    println!("The largest prime factor of {}, is {},", 600851475143 as u128, problem_0003::largest_prime_factor(600851475143 ));
    println!("The largest palindorme product of two {}-digit factors, is {},", 3, problem_0004::largest_palindrome_product(3));
    //println!("The smallest integer divisible by numbers 1 to {}, is {}", 20, problem_0005::smallest_multiple(20));
    println!("The difference between sum of squares and square of summes for 1 to {}, is {}.", 100, problem_0006::sum_square_difference(100));
    println!("The 10001st prime is {}.", problem_0007::nth_prime(10001));
    println!("The largest product in a series for the hardcoded number for {} digits is {:?}.", 13, problem_0008::largest_product_in_a_series(13));
    //println!("The special pythagorean triplet with a+b+c=1000 is with product {:?}.", problem_0009::special_pythagorean_triplet(1000));
    println!("Sum of all primes lower than {} is {}.", 2_000_000, problem_0010::summation_of_primes(2_000_000));
    println!("The first triangle number to have over {} divisors is {}.", 500, problem_0012::highly_divisible_triangle_number(500));
    println!("The longest collatz sequence for positive integers smaller {} has {}.", 1_000_000, problem_0014::longest_collatz_sequence(1_000_000));
    //println!("The sum of the digits in {}! (factorial) is {}.", 100, problem_0020::factorial_digit_sum(100));
    println!("The sum of all amicable numbers below {} is {}.", 10000, problem_0021::amicable_numbers(10000));
    println!("The sum of all numbers, which can be written as the sum of fifth powers of their digits, is {}.", problem_0030::digit_fifth_power());
}
