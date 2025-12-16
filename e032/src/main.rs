//! We shall say that an n-digit number is pandigital if
//! it makes use of all the digits 1 to n exactly once;
//! for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//! The product 7254 is unusual, as the identity,
//! 39 × 186 = 7254,
//! containing multiplicand, multiplier, and product is 1 through 9 pandigital.
//! Find the sum of all products whose multiplicand/multiplier/product
//! identity can be written as a 1 through 9 pandigital.
//! HINT: Some products can be obtained in more than one way
//! so be sure to only include it once in your sum.

use std::collections::HashSet;
use utils::misc;

fn answer() -> u64 {
    let mut digit_options = Vec::new();
    for n in 1..=9 {
        digit_options.push(n);
    }

    let mut pandigital_prod = HashSet::new();

    for a in 1..100 {
        for b in 1..10_000 / a {
            // We only really want 4 digit products
            // made from either 1 x 4 digit factors or
            // 2 x 3 digit factors
            let c = a * b;
            let mut digits =
                [misc::digits_of(a), misc::digits_of(b), misc::digits_of(c)]
                    .concat();
            digits.sort();
            if digits == digit_options {
                pandigital_prod.insert(c);
            }
        }
    }

    println!(
        "Find the sum of all products whose multiplicand/multiplier/product"
    );
    println!("identity can be written as a 1 through 9 pandigital.");

    println!("\nProducts: {:?}", &pandigital_prod);

    pandigital_prod.iter().sum()
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e032_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 45_228;
        assert_eq!(expected, answer());
    }
}
