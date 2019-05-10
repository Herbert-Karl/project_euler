//! module for (possibly) commonly used functions in the solutions 

pub fn fibonacci(n: u128) -> u128 {
    let mut x: (u128, u128) = (1, 1);
    for _ in 0..n {
        x = (x.1, x.0 + x.1);
    }
    x.0
}
