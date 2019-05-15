//! module for (possibly) commonly used functions in the solutions

pub fn fibonacci(n: u128) -> u128 {
    let mut x: (u128, u128) = (1, 1);
    for _ in 0..n {
        x = (x.1, x.0 + x.1);
    }
    x.0
}

/// returns the next smaller integer value to the square root of a given number
/// by casting the integer to a float and back after sqrt
pub fn integer_sqrt(x: u128) -> u128 {
    (x as f64).sqrt().floor() as u128
}

pub fn is_prime(x: u128) -> bool {
    if x==2 {
        return true
    }
    let help = integer_sqrt(x);
    if x<2 || x%2==0 {
        false
    } else {
        let mut i: u128 = 3;
        while i<=help {
            if x%i==0 {
                return false
            }
            i = i+2;
        };
        true
    }
}

pub fn is_palindrome(n: u128) -> bool {
    let original = n.to_string();
    let reverse = original.chars().rev().collect::<String>();   // reversing through use of a iterator of the chars of the string
    original.eq(&reverse)
}

/// returns the collatz sequence starting from n as Vector
///
/// definition of the iterative sequence
/// * n even -> use n/2 next
/// * n odd -> use 3*n+1 next
///
/// the sequence does contain n and 1
pub fn collatz_sequence(n: u64) -> Vec<u64> {
    if n==0 {
        return Vec::new()
    }
    let mut sequence = vec!(n);
    let mut i = n;
    while i!=1 {
        if i%2==0 { // even
            i = i/2;
        } else {    // odd
            i = 3*i+1
        }
        sequence.push(i);
    }
    sequence
}

pub fn factorial(n: u128) -> u128 {
    if n==0 {
        1
    } else {
        (1..n+1).into_iter().product()
    }
}

/// computes, if n is part of a amicable pair
///
/// if yes, it returns the amicable numbers as Some(tuple) starting with N
/// if no, it returns None
pub fn amicable_pair(n: u128) -> Option<(u128, u128)> {
    let sum_proper_divisors = proper_divisors(n).into_iter().sum();
    if n==proper_divisors(sum_proper_divisors).into_iter().sum() && n!=sum_proper_divisors {
        Some((n, sum_proper_divisors))
    } else {
        None
    }
}

/// proper divisors of N are numbers less then N which divide evenly into N
pub fn proper_divisors(n: u128) -> Vec<u128> {
    let mut dividers = vec!(1);
    for i in 2..integer_sqrt(n)+1 {
        if n%i==0 {
            dividers.push(i);
            dividers.push(n/i);
        }
    }
    dividers
}
