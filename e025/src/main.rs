//! The Fibonacci sequence is defined by the recurrence relation:
//! Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//! Hence the first 12 terms will be:
//! F1 = 1
//! F2 = 1
//! F3 = 2
//! F4 = 3
//! F5 = 5
//! F6 = 8
//! F7 = 13
//! F8 = 21
//! F9 = 34
//! F10 = 55
//! F11 = 89
//! F12 = 144
//! The 12th term, F12, is the first term to contain three digits.
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use utils::big::StrInteger;

fn answer() -> u64 {
    let mut fib_1 = StrInteger::from(1);
    let mut fib_2 = StrInteger::from(1);
    
    let target_digits = 1_000;
    
    let mut idx = 3;
    let mut fib_i = fib_1.plus(&fib_2);
    while fib_i.digits() < target_digits {
        idx += 1;
        fib_2 = fib_1.clone();
        fib_1 = fib_i.clone();
        fib_i = fib_1.plus(&fib_2);
    }

    println!("What is the index of the first term in the Fibonacci sequence to contain {} digits?", &target_digits);
    println!("Index: {}    Term: {}", &idx, &fib_i);

    idx
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e025_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 4782;
        assert_eq!(expected, answer());
    }
}
