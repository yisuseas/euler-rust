//! n! means n × (n − 1) × ... × 3 × 2 × 1
//! For example,
//! 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number
//! 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//! Find the sum of the digits in the number 100!

use utils::{big::U1024, misc::char_to_u8};

fn factorial(n: U1024) -> U1024 {
    if n == U1024::ONE {
        return n;
    }
    n * factorial(n - U1024::ONE)
}

fn char_to_u64(ch: char) -> u64 {
    char_to_u8(ch) as u64
}

fn answer() -> u64 {
    let n = U1024::from_small(100);
    let n_fac = factorial(n);
    println!("{}! =\n{}", &n, &n_fac);

    n_fac
        .to_string()
        .chars()
        .map(char_to_u64)
        .sum()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e020_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 648;
        assert_eq!(expected, answer());
    }
}
