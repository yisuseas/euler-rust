//! 145 is a curious number, as
//!     1! + 4! + 5! = 1 + 24 + 120 = 145.
//! Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//! Note: As 1! = 1 and 2! = 2 are not sums they are not included.

use utils::misc;

fn sum_fac_digits(n: u64, fac: &[u64]) -> u64 {
  misc::digits_of(n)
    .iter()
    .fold(0, |sum, &d| sum + fac[d as usize])
}

fn answer() -> u64 {
  let fac = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];
  // Find max value
  let mut max_val = 0;
  for digits in 1..100 {
    let max = (0..digits).fold(0, |sum, exp| sum + 9 * 10u64.pow(exp));
    let sfd = sum_fac_digits(max, &fac);
    if max > sfd {
      max_val = sfd;
      break;
    }
  }

  println!("Find the sum of all numbers which are equal");
  println!("to the sum of the factorial of their digits.\n");

  let mut sum = 0;
  for n in 10..max_val {
    let sfd = sum_fac_digits(n, &fac);
    if n == sfd {
      sum += n;
      println!("{}  ", n);
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
mod e034_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 40730;
    assert_eq!(expected, answer());
  }
}
