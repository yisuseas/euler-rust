//! The number, 197, is called a circular prime because all rotations of the digits:
//!     197, 971, 719,
//! are themselves prime.
//! There are thirteen such primes below 100:
//!     2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//! How many circular primes are there below one million?

use std::collections::HashSet;

use utils::{misc, primes};

fn rotations(n: usize) -> Vec<usize> {
  let digits = misc::digits_of(n);
  let mut rot = Vec::new();
  for start_idx in 0..digits.len() {
    let mut s = String::new();
    for addend in 0..digits.len() {
      let idx = (start_idx + addend) % digits.len();
      s.push(misc::u8_to_char(digits[idx]));
    }
    rot.push(s.parse().unwrap());
  }
  rot
}

fn answer() -> usize {
  let target = 1_000_000;
  let prime_list = primes::primes_under(target);
  let prime_set: HashSet<_> = HashSet::from_iter(prime_list.iter());

  let mut circular = HashSet::new();
  let mut not_circular = HashSet::new();
  for &prime in prime_list.iter() {
    // If we know skip
    if not_circular.contains(&prime) || circular.contains(&prime) {
      continue;
    }
    // Find out if the rotations are circular
    let mut are_circular = true;
    let rotation_list = rotations(prime);
    for rotation in rotation_list.iter() {
      if !prime_set.contains(rotation) {
        are_circular = false;
        break;
      }
    }
    // Store all rotations in the appropiate set
    if are_circular {
      for rotation in rotation_list {
        circular.insert(rotation);
      }
    } else {
      for rotation in rotation_list {
        not_circular.insert(rotation);
      }
    }
  }

  println!("How many circular primes are there below {}?", target);

  let mut circular: Vec<&usize> = circular.iter().collect();
  circular.sort();
  println!("\n{:?}", &circular);

  circular.len()
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e035_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 55;
    assert_eq!(expected, answer());
  }
}
