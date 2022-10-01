//! Euler discovered the remarkable quadratic formula:
//! n^2 + n + 41
//! It turns out that the formula will produce 40 primes for the
//! consecutive integer values.
//! 0 <= n <= 39
//! However, when
//! n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41
//! is divisible by 41, and certainly when
//! n = 41, 41^2 + 41 + 41
//! is clearly divisible by 41.
//! The incredible formula
//! n^2 - 79n + 1601
//! was discovered, which produces 80 primes for the consecutive values
//! 0 <= n <= 79
//! The product of the coefficients, −79 and 1601, is −126479.
//! Considering quadratics of the form:
//! n^2 + an + b , where |a| < 1000 and |b| <= 1000
//! where |n| is the modulus/absolute value of n
//! e.g. |11| = 11 and |-4| = 4
//! Find the product of the coefficients, a and b,
//! for the quadratic expression that produces the maximum number
//! of primes for consecutive values of n, starting with n = 0.

fn is_prime(n: i32) -> bool {
  if n <= 0 || n % 2 == 0 {
    return false;
  }
  let mut div = 3;
  while div * div <= n + 1 {
    if n % div == 0 {
      return false;
    }
    div += 2;
  }
  true
}

fn quadratic_expr(n: i32, a: i32, b: i32) -> i32 {
  n * n + a * n + b
}

fn primes_found_with(a: i32, b: i32) -> i32 {
  let mut n = -1;
  loop {
    n += 1;
    let x = quadratic_expr(n, a, b);
    if !is_prime(x) {
      break;
    }
  }
  n
}

fn answer() -> i32 {
  let target = 1_000;

  let mut most_primes = 0;
  let mut best_a = 0;
  let mut best_b = 0;

  for a in (-target + 1)..target {
    for b in -target..=target {
      let prime_quant = primes_found_with(a, b);
      if prime_quant > most_primes {
        most_primes = prime_quant;
        best_a = a;
        best_b = b;
      }
    }
  }

  println!("Considering quadratics of the form:");
  println!("n^2 + an + b , where |a| < 1000 and |b| <= 1000");
  println!("Find the product of the coefficients, a and b,");
  println!("for the quadratic expression that produces the maximum number");
  println!("of primes for consecutive values of n, starting with n = 0.\n");

  println!(
    "a: {}, b: {}\nprimes found: {}",
    &best_a, &best_b, most_primes
  );

  best_a * best_b
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e027_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = -59231;
    assert_eq!(expected, answer());
  }
}
