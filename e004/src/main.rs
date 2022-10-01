// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is
// 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use utils::misc;

fn answer() -> u64 {
  let digit_count: u32 = 3;

  let start: u64 = 10u64.pow(digit_count - 1);
  let end: u64 = 10u64.pow(digit_count);

  let mut largest_palindrome = 0;

  let mut number_1 = end - 1;
  while number_1 >= start {
    let mut number_2 = end - 1;
    while number_2 >= start {
      let product = number_1 * number_2;
      if product > largest_palindrome {
        if misc::is_palindromic(&product) {
          largest_palindrome = product;
        }
      }
      number_2 -= 1;
    }
    number_1 -= 1;
  }

  println!(
        "Find the largest palindrome made from the product of two {}-digit numbers.",
        digit_count,
    );

  largest_palindrome
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e004_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 906609;
    assert_eq!(expected, answer());
  }
}
