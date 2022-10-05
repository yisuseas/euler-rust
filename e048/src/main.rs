//! The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317
//! Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.

use num_bigint::BigUint;

fn answer() -> String {
  let limit = 1_000u16;
  let mut sum = BigUint::from(0u16);

  for n in 1..=limit {
    let factor = BigUint::from(n);
    let mut term = BigUint::from(1u16);
    for _ in 0..n {
      term *= &factor;
    }
    sum += term;
  }

  println!("The series, 1^1 + 2^2 + ... + 10^10 = 10405071317");
  println!("Find the last ten digits of the series ");
  println!("1^1 + 2^2 + ... + 1000^1000.");

  let sum_string = sum.to_string();
  String::from(&sum_string[sum_string.len() - 10..])
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e048_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = "9110846700";
    assert_eq!(expected, answer());
  }
}
