//! for solving the problems from project euler <https://projecteuler.net/>

mod common;
#[cfg(test)]
mod tests;

use crate::common::*;

fn main() {
    println!("Welcome to \"Solving Projecc Euler\"!");
    println!("Multiples of three and five for n = {}, is {}", 1000, multiples_of_three_and_five(1000));
    println!("The sum of all even fibonacci numbers till the {}th smaller 4 mil, is {}", 100, even_fibonacci_numbers(100));
    println!("The largest prime factor of {}, is {}", 600851475143 as u128, largest_prime_factor(600851475143 ));
    println!("The largest palindorme product of two {}-digit factors, is {}", 3, largest_palindrome_product(3));
}

/// problem 1
///
/// sum of all multiples of three and five below N
fn multiples_of_three_and_five(n: u64) -> u64 {
    (1..n).into_iter().filter(|x| x%3==0 || x%5==0).sum()
}

/// problem 2
///
/// sum of all even fibonacci numbers till N smaller then 4_000_000
fn even_fibonacci_numbers(n: u128) -> u128 {
    // because i dont know, till which fibonacci number, they are smaller 4 mil, i just create a big enough range to iterate over
    (0..n).into_iter().map(|x| fibonacci(x)).filter(|&x| x<4_000_000 && x%2==0).sum()
}


/// problem 3
///
/// largest prime factor of given number N
fn largest_prime_factor(n: u128) -> u128 {
    let mut list: Vec<u128> = (2..integer_sqrt(n)).into_iter().filter(|&x| n%x==0 && is_prime(x)).collect();
    list.pop().unwrap_or_else(|| {println!("No prime factors exist for {}! Returning zero.", n); 0})
}

/// problem 4
///
/// largest palindrome product of two n-digit numbers
fn largest_palindrome_product(n: u32) -> u128 {
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
