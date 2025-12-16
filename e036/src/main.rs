//! The decimal number,
//! 585 = 10010010012 (binary),
//! is palindromic in both bases.
//! Find the sum of all numbers, less than one million,
//! which are palindromic in base 10 and base 2.
//!
//! (Please note that the palindromic number,
//! in either base, may not include leading zeros.)

use utils::misc;

fn answer() -> u64 {
    let target = 1_000_000;

    println!("Find the sum of all numbers, less than one million,");
    println!("which are palindromic in base 10 and base 2.\n");

    let mut sum = 0;
    for num in 1..target {
        if misc::is_palindromic(&num) {
            let bin = format!("{num:b}");
            if misc::is_palindromic(&bin) {
                println!("{num} -> {bin}");
                sum += num;
            }
        }
    }

    sum
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e036_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 872_187;
        assert_eq!(expected, answer());
    }
}
