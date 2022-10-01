//! Take the number 192 and multiply it by each of 1, 2, and 3:
//!     192 × 1 = 192
//!     192 × 2 = 384
//!     192 × 3 = 576
//! By concatenating each product we get the 1 to 9 pandigital, 192384576.
//! We will call 192384576 the concatenated product of 192 and (1,2,3)
//! The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
//! giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
//! What is the largest 1 to 9 pandigital 9-digit number that can be
//! formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

use utils::misc;

fn concatenated(seed: u64, n: u64) -> u64 {
  let mut s = String::new();
  for mult in 1..=n {
    s.push_str(&(seed * mult).to_string());
  }
  s.parse().unwrap()
}

fn is_pandigital(n: u64) -> bool {
  let mut digits = misc::digits_of(n);
  digits.sort();
  digits == [1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn answer() -> u64 {
  println!("What is the largest 1 to 9 pandigital 9-digit number that can be");
  println!("formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?\n");

  // Max seed would be a 4 digit number multiplied by (1-2)
  // n = 2    max_seed = 9_999
  // Min seed would be a 1 digit number multiplied by (1-9)
  // n = 9    min_seed = 1
  let mut largest_pandigital = 0;
  for seed in 1..=9_999 {
    for n in 2..9 {
      let con_prod = concatenated(seed, n);
      if con_prod >= 1_000_000_000 {
        break;
      }
      if is_pandigital(con_prod) {
        println!("{seed} * (1-{n}) -> {con_prod}");
        if con_prod > largest_pandigital {
          largest_pandigital = con_prod;
        }
      }
    }
  }

  largest_pandigital
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e038_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 932718654;
    assert_eq!(expected, answer());
  }
}
