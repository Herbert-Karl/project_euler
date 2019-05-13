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
