//! Let d(n) be defined as the sum of proper divisors of n
//! (numbers less than n which divide evenly into n).
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b
//! are an amicable pair and each of a and b are called amicable numbers.
//! For example,
//! the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
//! therefore d(220) = 284.
//! The proper divisors of 284 are 1, 2, 4, 71 and 142;
//! so d(284) = 220.
//! Evaluate the sum of all the amicable numbers under 10000.

use std::collections::HashSet;
use utils::misc;

fn proper_div_sum(n: usize) -> usize {
  misc::proper_divisors_of(n).iter().sum()
}

fn answer() -> u64 {
  let target = 10_000;
  let mut amicable = HashSet::new();

  for a in 2..target {
    let b = proper_div_sum(a);
    if b > a {
      if proper_div_sum(b) == a {
        amicable.insert(a);
        amicable.insert(b);
      }
    }
  }

  println!(
    "The amicable numbers under {} are:\n{:?}",
    &target, &amicable
  );
  println!("Evaluate their sum.");

  amicable.iter().fold(0, |sum, &ami| sum + ami as u64)
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e021_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 31626;
    assert_eq!(expected, answer());
  }
}
