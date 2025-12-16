//! Surprisingly there are only three numbers that can be written as
//! the sum of fourth powers of their digits:
//!     1634 = 1^4 + 6^4 + 3^4 + 4^4
//!     8208 = 8^4 + 2^4 + 0^4 + 8^4
//!     9474 = 9^4 + 4^4 + 7^4 + 4^4
//! As 1 = 1^4 is not a sum it is not included.
//! The sum of these numbers is
//!     1634 + 8208 + 9474 = 19316.
//! Find the sum of all the numbers that can be written as the
//! sum of fifth powers of their digits.

use utils::misc;

fn sum_pow_digits(n: u64, exp: u32) -> u64 {
    // max pos value = 9^exp
    misc::digits_of(n)
        .iter()
        .fold(0, |sum, &d| sum + (d as u64).pow(exp))
}

fn answer() -> u64 {
    let exp = 5;
    // Find limit
    let mut digits = 2;
    loop {
        let max_val = 9u64.pow(exp);
        if 10u64.pow(digits) > max_val * (digits as u64) {
            break;
        }
        digits += 1;
    }
    // Find the numbers
    let mut found_numbers = vec![];
    for n in 10..10u64.pow(digits) {
        if n == sum_pow_digits(n, exp) {
            found_numbers.push(n);
        }
    }

    println!("Find the sum of all the numbers that can be written as the");
    println!(
        "sum of {} powers of their digits.",
        misc::ordinal_number(exp as usize)
    );

    println!("\n{:?}", &found_numbers);

    found_numbers.iter().sum()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e030_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 443_839;
        assert_eq!(expected, answer());
    }
}
