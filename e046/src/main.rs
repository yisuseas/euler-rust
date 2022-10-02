//! It was proposed by Christian Goldbach that
//! every odd composite number can be written
//! as the sum of a prime and twice a square.
//! 9  = 7  + 2×1^2
//! 15 = 7  + 2×2^2
//! 21 = 3  + 2×3^2
//! 25 = 7  + 2×3^2
//! 27 = 19 + 2×2^2
//! 33 = 31 + 2×1^2
//! It turns out that the conjecture was false.
//! What is the smallest odd composite that cannot be
//! written as the sum of a prime and twice a square?

use std::collections::HashSet;

use utils::primes;

#[derive(Debug)]
struct NumberCache {
  prime_vec: Vec<usize>,
  prime_set: HashSet<usize>,
  biggest: usize,
}

impl NumberCache {
  fn new() -> Self {
    let prime_vec = primes::primes_under(1_000_000);
    let prime_set = HashSet::from_iter(prime_vec.iter().map(|&p| p));
    let &biggest = prime_vec.last().unwrap();
    NumberCache {
      prime_vec,
      prime_set,
      biggest,
    }
  }

  fn is_prime(&self, n: usize) -> bool {
    if n > self.biggest {
      panic!("Went over limit");
    }
    self.prime_set.contains(&n)
  }

  fn follows_conjecture(&self, n: usize) -> bool {
    for &prime in self.prime_vec.iter() {
      if prime >= n {
        break;
      }
      let remainder = n - prime;
      if remainder % 2 == 0 {
        let square = remainder / 2;
        let x = (square as f32).sqrt() as usize;
        if x * x == square {
          return true;
        }
      }
    }
    false
  }
}

fn answer() -> usize {
  let c = NumberCache::new();

  println!("What is the smallest odd composite that cannot be");
  println!("written as the sum of a prime and twice a square?");

  let mut n = 3;
  loop {
    if !c.is_prime(n) {
      // If not 1 or prime is composite
      if !c.follows_conjecture(n) {
        return n;
      }
    }
    n += 2;
  }
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e046_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 5777;
    assert_eq!(expected, answer());
  }
}
