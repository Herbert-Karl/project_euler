//! module containing problem 1

/// problem 1
///
/// sum of all multiples of three and five below N
pub fn multiples_of_three_and_five(n: u64) -> u64 {
    (1..n).into_iter().filter(|x| x%3==0 || x%5==0).sum()
}
