//! A perfect number is a number for which the sum of its
//! proper divisors is exactly equal to the number.
//! For example, the sum of the proper divisors of 28 would be
//! 1 + 2 + 4 + 7 + 14 = 28,
//! which means that 28 is a perfect number.
//! A number n is called deficient if the sum of its proper divisors
//! is less than n and it is called abundant if this sum exceeds n.
//! As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16,
//! the smallest number that can be written as the sum of two abundant numbers is 24.
//! By mathematical analysis, it can be shown that all integers
//! greater than 28123 can be written as the sum of two abundant numbers.
//! However, this upper limit cannot be reduced any further by analysis
//! even though it is known that the greatest number that cannot be expressed as
//! the sum of two abundant numbers is less than this limit.
//!
//! Find the sum of all the positive integers which
//! cannot be written as the sum of two abundant numbers.

use std::collections::HashSet;
use utils::misc;

fn is_abundant(n: u64) -> bool {
    let n_u = n as usize;
    n_u < misc::proper_divisors_of(n_u).iter().sum()
}

fn answer() -> u64 {
    // Find Abundant Numbers
    let target: u64 = 28_123;
    let mut abundant = Vec::new();
    for n in 1..=target {
        if is_abundant(n) {
            abundant.push(n);
        }
    }
    // Make numbers from them
    let mut from_two_abundant = HashSet::new();
    for i in 0..abundant.len() {
        for j in i..abundant.len() {
            let sum = abundant[i] + abundant[j];
            from_two_abundant.insert(sum);
        }
    }

    println!("Find the sum of all the positive integers which");
    println!("cannot be written as the sum of two abundant numbers.");

    (1..=target)
        .filter(|n| !from_two_abundant.contains(n))
        .sum()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e023_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 4179871;
        assert_eq!(expected, answer());
    }
}
