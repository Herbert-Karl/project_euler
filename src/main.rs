//! for solving the problems from project euler <https://projecteuler.net/>

fn main() {
    println!("Welcome to \"Solving Projecc Euler\"!");
    println!("Multiples of three and five for n = {}, is {}", 1000, multiples_of_three_and_five(1000));
    println!("The sum of all even fibonacci numbers till the {}th smaller 4 mil, is {}", 100, even_fibonacci_numbers(100));
}

/// problem 1
///
/// sum of all multiples of three and five below X
fn multiples_of_three_and_five(n: u64) -> u64 {
    (1..n).into_iter().filter(|x| x%3==0 || x%5==0).sum()
}

fn fibonacci(n: u128) -> u128 {
    let mut x: (u128, u128) = (1, 1);
    for _ in 0..n {
        x = (x.1, x.0 + x.1);
    }
    x.0
}

/// problem 2
///
/// sum of all even fibonacci numebrs smaller then 4_000_000
fn even_fibonacci_numbers(n: u128) -> u128 {
    (0..n).into_iter().map(|x| fibonacci(x)).filter(|&x| x<4_000_000 && x%2==0).sum()
}
