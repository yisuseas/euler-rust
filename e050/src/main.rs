//! The prime 41, can be written as the sum of six consecutive primes:
//! 41 = 2 + 3 + 5 + 7 + 11 + 13
//! This is the longest sum of consecutive primes
//! that adds to a prime below one-hundred.
//! The longest sum of consecutive primes below one-thousand
//! that adds to a prime, contains 21 terms, and is equal to 953.
//! Which prime, below one-million, can be written as
//! the sum of the most consecutive primes?

use std::collections::HashSet;

use utils::primes;

fn answer() -> usize {
  let limit = 1_000_000;
  let prime_vec = primes::primes_under(limit);
  let prime_set: HashSet<&usize> = HashSet::from_iter(prime_vec.iter());

  println!("Which prime, below {limit}, can be written as");
  println!("the sum of the most consecutive primes?");

  // Find max possible length
  let mut sum = 0;
  let mut max_len = prime_vec.len();
  for (idx, &prime) in prime_vec.iter().enumerate() {
    sum += prime;
    if sum >= limit {
      max_len = idx + 1;
      break;
    }
  }

  // Find chain
  for len in (1..max_len).rev() {
    for start in 0..=prime_vec.len() - len {
      let prime_slice = &prime_vec[start..start + len];
      let sum: usize = prime_slice.iter().sum();
      if sum > limit {
        break;
      }
      if prime_set.contains(&sum) {
        println!(
          "\nConsecutive primes: {} to {}\nSequence length: {}",
          prime_slice.first().unwrap(),
          prime_slice.last().unwrap(),
          len
        );
        return sum;
      }
    }
  }

  0
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e050_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 997_651;
    assert_eq!(expected, answer());
  }
}
