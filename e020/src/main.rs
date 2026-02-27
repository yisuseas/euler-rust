//! n! means n × (n − 1) × ... × 3 × 2 × 1
//! For example,
//! 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number
//! 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//! Find the sum of the digits in the number 100!

use utils::{big::U1024, misc::ToFromChar};

fn factorial(n: U1024) -> U1024 {
    if n == U1024::ONE {
        return n;
    }
    n * factorial(n - U1024::ONE)
}

fn answer() -> u64 {
    let n = U1024::from_small(100);
    let n_fac = factorial(n);
    println!("{}! =\n{}", &n, &n_fac);

    n_fac.to_string().chars().map(u64::from_char).sum()
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
