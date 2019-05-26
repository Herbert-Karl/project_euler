//! module containing problem 30

/// problem 30
///
/// computes the sum of all numbers that can be written as the sum of fifth powers of their digits.
///
/// the upper bound for this computation has to be derived:
/// * as 9 is the biggest digit, 9^5 is our biggest summand
/// * we need the upper limit of digits, so that a sum of its digit to the fifth power still creates a number with the same amount of digits.
/// This is 6, as 7*9^5 = 413343, which is not a 7 digit number, so 7 digit numbers cant be written as the sum of the fifth power of their digits
pub fn digit_fifth_power() -> u32 {
    let upper_bond = 6*9_u32.pow(5);
    let mut vector = Vec::new();
    for i in 2..upper_bond+1 {
        if i == i.to_string().chars().map(|x| x.to_digit(10).expect("this is no digit!").pow(5)).sum() {
            vector.push(i);
        }
    }
    vector.into_iter().sum()
}
