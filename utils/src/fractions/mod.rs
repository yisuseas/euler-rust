//! Rational numbers related functions and struct

use super::primes;
use std::collections::HashMap;

/// Struct containing a numerator and denominator, both u64
///
/// PartialEq is made comparing both num and den.
/// two equivalent fractions in value will compare as diferent.
/// PartialOrd is evaluated from their decimal representation.
///
/// Also implements the following operators:
///
/// +, +=, -, -=, *, *=, /, /=
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    pub num: u64,
    pub den: u64,
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Fraction {
    pub fn new(num: u64, den: u64) -> Fraction {
        Fraction { num, den }
    }

    /// Unit fraction: 1 / den
    pub fn unit(den: u64) -> Fraction {
        Fraction { num: 1, den }
    }

    /// Decimal value of the fraction as f64.
    ///
    /// Use with caution, susceptible to floating point errors
    pub fn decimal(&self) -> f64 {
        ((1_000_000.0 * self.num as f64) / self.den as f64) / 1_000_000.0
    }

    /// Returns an array of the first 100 decimal digits
    ///
    /// Ignores the whole part if that exists
    pub fn decimal_digits(&self) -> [u8; 100] {
        let mut digit_list = [0; 100];
        let mut rem = self.num % self.den;
        for digit in digit_list.iter_mut() {
            *digit = (rem * 10 / self.den) as u8;
            rem = rem * 10 % self.den;
        }
        digit_list
    }

    /// Returns a vector with the digits of the recurring cycle
    /// of the decimal interpretation.
    /// will be empty if there's no cycle
    pub fn recurring_cycle(&self) -> Vec<u8> {
        let mut div_rem_vec = vec![];

        let mut rem = self.num % self.den;
        loop {
            let div = (rem * 10 / self.den) as u8;
            rem = rem * 10 % self.den;
            // is not recurring
            if rem == 0 {
                return vec![];
            }
            let div_rem = (div, rem);
            if div_rem_vec.contains(&div_rem) {
                // we got a recurring cycle
                let mut cycle = vec![];
                let mut found_start = false;
                for &d_r in div_rem_vec.iter() {
                    if d_r == div_rem {
                        found_start = true;
                    }
                    if found_start {
                        cycle.push(d_r.0);
                    }
                }
                return cycle;
            } else {
                div_rem_vec.push(div_rem);
            }
        }
    }

    /// Will simplify the fraction by applying prime factorization
    pub fn simplify(&mut self) {
        let mut num_f = primes::prime_factors_hm(self.num as usize);
        let mut den_f = primes::prime_factors_hm(self.den as usize);

        let mut common_factors_smallest = HashMap::new();
        for (&fac, &n_exp) in num_f.iter() {
            if let Some(&d_exp) = den_f.get(&fac) {
                common_factors_smallest.insert(fac, n_exp.min(d_exp));
            }
        }

        for (fac, smallest) in common_factors_smallest {
            if let Some(exp) = num_f.get_mut(&fac) {
                *exp -= smallest;
            }
            if let Some(exp) = den_f.get_mut(&fac) {
                *exp -= smallest;
            }
        }

        self.num = num_f
            .iter()
            .map(|(&f, &e)| (f as u64, e))
            .fold(1, |prod, (f, e)| prod * f.pow(e));
        self.den = den_f
            .iter()
            .map(|(&f, &e)| (f as u64, e))
            .fold(1, |prod, (f, e)| prod * f.pow(e));
    }

    /// Returns as a new fraction the simplified version of self
    ///
    /// It makes a deep copy, since its only two u64 values
    pub fn simplified(&self) -> Fraction {
        let mut s = *self;
        s.simplify();
        s
    }
}

mod operators;

////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formating() {
        let f = Fraction::new(1, 2);
        assert_eq!(String::from("1/2"), f.to_string());
    }

    #[test]
    fn operators() {
        let a = Fraction::new(3, 4);
        let b = Fraction::new(2, 3);
        assert!(a != b);
        assert!(a > b);
        assert_eq!(Fraction::new(17, 12), a + b);
        assert_eq!(Fraction::new(6, 12), a * b);
        assert_eq!(Fraction::new(1, 12), a - b);
        assert_eq!(Fraction::new(9, 8), a / b);
    }

    #[test]
    fn simplification() {
        let a = Fraction::new(1, 3);
        let mut b = Fraction::new(5, 15);
        assert_ne!(a, b);
        b.simplify();
        assert_eq!(a, b);
        // 10 / 33
        let a = Fraction::new(2 * 5, 3 * 11);
        // 140 / 462
        let mut b = Fraction::new(2 * 2 * 5 * 7, 2 * 3 * 7 * 11);
        assert_ne!(a, b);
        b.simplify();
        assert_eq!(a, b);
    }

    #[test]
    fn ordering() {
        let a = Fraction::new(3, 4);
        let b = Fraction::new(15, 20);
        assert!(a <= b);
        assert!(a >= b);
        assert_ne!(a, b);
        assert_eq!(a, b.simplified());
    }
}
