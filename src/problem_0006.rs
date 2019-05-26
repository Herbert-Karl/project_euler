//! module containing problem 6

/// problem 6
///
/// difference between the sum of the squares of the natural numbers till N and the square of the sum of the natural numbers till N
pub fn sum_square_difference(n: u128) -> u128 {
    let sum_squares = (1..n+1).into_iter().map(|x| x.pow(2)).sum::<u128>();
    let square_sum = (1..n+1).into_iter().sum::<u128>().pow(2);
    square_sum-sum_squares
}
