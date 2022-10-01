//! The fraction 49/98 is a curious fraction,
//! as an inexperienced mathematician in attempting to simplify it
//! may incorrectly believe that 49/98 = 4/8, which is correct,
//! is obtained by cancelling the 9s.
//! We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//! There are exactly four non-trivial examples of this type of fraction,
//! less than one in value, and containing two digits in the numerator and denominator.
//! If the product of these four fractions is given in its lowest common terms,
//! find the value of the denominator.

use utils::fractions::Fraction;

fn n_from_digits(digits: &[Option<u64>; 2]) -> Option<u64> {
  match digits {
    [None, None] => None,
    [dec, uni] => {
      let mut s = String::new();
      if let &Some(x) = dec {
        s.push_str(&x.to_string());
      }
      if let &Some(x) = uni {
        s.push_str(&x.to_string());
      }
      Some(s.parse().unwrap())
    }
  }
}

fn answer() -> u64 {
  let mut curious = Vec::new();

  println!("Curious fractions:");
  for den in 10..100 {
    // I dont want trivial 10/20 kind of cases or to divide by 0
    if den % 10 == 0 {
      continue;
    }
    for num in 10..den {
      let mut num_digits = [Some(num / 10), Some(num % 10)];
      let mut den_digits = [Some(den / 10), Some(den % 10)];
      // Cancel common digits
      for num_digit in num_digits.iter_mut() {
        for den_digit in den_digits.iter_mut() {
          if num_digit == den_digit {
            *num_digit = None;
            *den_digit = None;
          }
        }
      }
      // If the numbers are valid, chech the fractions
      if let Some(simple_num) = n_from_digits(&num_digits) {
        if let Some(simple_den) = n_from_digits(&den_digits) {
          let frac = Fraction::new(num, den);
          let real_simpl = Fraction::new(num, den).simplified();
          let novice_simpl = Fraction::new(simple_num, simple_den);
          if frac != real_simpl && frac != novice_simpl {
            if real_simpl.decimal() == novice_simpl.decimal() {
              println!("{}/{}", num, den);
              curious.push(frac);
            }
          }
        }
      }
    }
  }

  println!("\nIf the product of these four fractions is given in its lowest common terms,");
  println!("find the value of the denominator.");

  let product = curious
    .iter()
    .fold(Fraction::unit(1), |prod, &frac| prod * frac)
    .simplified();
  println!("\nTheir product is: {}", &product);

  product.den
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e033_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 100;
    assert_eq!(expected, answer());
  }
}
