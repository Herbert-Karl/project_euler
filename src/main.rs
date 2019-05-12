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
    //println!("The smallest integer divisible by numbers 1 to {}, is {}", 20, smallest_multiple(20));
    println!("The difference between sum of squares and square of summes for 1 to {}, is {}", 100, sum_square_difference(100));
    println!("The 10001st prime is {}", nth_prime(10001));
    println!("The largest product in a series for the hardcoded number for {} digits is {:?}", 13, largest_product_in_a_series(13));
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

/// problem 5
///
/// smallest positive number, that is evenly divisible by numbers 1 to N
fn smallest_multiple(n: u128) -> u128 {
    let mut num = n;
    while num<=(1..n+1).into_iter().product() {
        if (1..n+1).into_iter().all(|x| num%x==0) {
            break;
        } else {
            num = num+1;
        }
    }
    num
}


/// problem 6
///
/// difference between the sum of the squares of the natural numbers till N and the square of the sum of the natural numbers till N
fn sum_square_difference(n: u128) -> u128 {
    let sum_squares = (1..n+1).into_iter().map(|x| x.pow(2)).sum::<u128>();
    let square_sum = (1..n+1).into_iter().sum::<u128>().pow(2);
    square_sum-sum_squares
}

/// problem 7
///
/// calculates and returns the Nth prime number
fn nth_prime(n: u128) -> u128 {
    let mut x: (u128, u128) = (1, 2);
    while x.0<n {
        x.1 = x.1+1;
        if is_prime(x.1) {
            x.0 = x.0+1;
        }
    }
    x.1
}

/// problem 8
///
/// searches for the N adjacent digits in the 1000 digit number with the greatest product and returns digits+product
fn largest_product_in_a_series(n: usize) -> (u128, Vec<u8>) {
    let number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let mut i: usize = 0;
    let mut output: (u128, Vec<u8>) = (0, Vec::new());
    while i+n<number.len() {
        let slice = number.get(i..i+n).expect("Indexing went wrong");
        let product = slice.chars().map(|x| x.to_digit(10).expect("this is no digit!") as u128).product();
        if product>output.0 {
            output.1 = slice.chars().map(|x| x.to_digit(10).expect("this is no digit!") as u8).collect();
            output.0 = product;
        }
        i = i+1;
    }
    output
}
