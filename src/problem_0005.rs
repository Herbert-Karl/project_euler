//! module containing problem 5

/// problem 5
///
/// smallest positive number, that is evenly divisible by numbers 1 to N
pub fn smallest_multiple(n: u128) -> u128 {
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
