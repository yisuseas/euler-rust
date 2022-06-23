use super::*;

impl StrInteger {
    pub fn plus(&self, other: &StrInteger) -> StrInteger {
        let mut self_digits = self.n_str.chars().rev();
        let mut other_digits = other.n_str.chars().rev();
        
        let mut sum_str = String::new();
        let mut remainder = 0;
        
        loop {
            let a = self_digits.next();
            let b = other_digits.next();
            if a == None && b == None && remainder == 0 {
                break;
            }
            let a = a.unwrap_or('0').to_digit(10).unwrap() as u8;
            let b = b.unwrap_or('0').to_digit(10).unwrap() as u8;
            remainder += a + b;
            sum_str.push_str(&(remainder % 10).to_string());
            remainder /= 10;
        }
        
        StrInteger {
            n_str: sum_str.chars().rev().collect()
        }
    }

    pub fn times(&self, other: &StrInteger) -> StrInteger {
        let mut partial_mult_vec = Vec::new();
        other.n_str.chars()
             .map(|ch| ch.to_digit(10).unwrap() as u8)
             .rev()
             .enumerate()
             .for_each(|(other_idx, other_digit)|
        {
            let mut partial_mult = String::new();
            let mut remainder = 0;

            partial_mult.push_str(&("0".repeat(other_idx)));

            self.n_str.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .rev()
                .for_each(|self_digit|
            {
                remainder += other_digit * self_digit;
                partial_mult.push_str(&(remainder % 10).to_string());
                remainder /= 10;
            });

            while remainder > 0 {
                partial_mult.push_str(&(remainder % 10).to_string());
                remainder /= 10;
            }

            let n_str = partial_mult.chars().rev().collect::<String>();
            partial_mult_vec.push(StrInteger { n_str });
        });

        partial_mult_vec.iter().fold(
            StrInteger::zero(),
            |mult, partial| mult.plus(partial)
        )
    }

    /// Will panic if the substraction is negative
    pub fn minus(&self, other: &StrInteger) -> StrInteger {
        if self < other {
            panic!("Substraction would result in a negative number");
        } else if self == other {
            return StrInteger::zero();
        }
        
        let mut self_digits = self.n_str.chars().rev();
        let mut other_digits = other.n_str.chars().rev();

        let mut diff_str = String::new();
        let mut carry = 0;

        loop {
            let a = self_digits.next();
            let b = other_digits.next();
            if a == None && b == None {
                break;
            }
            let a = a.unwrap_or('0').to_digit(10).unwrap() as u8;
            let b = b.unwrap_or('0').to_digit(10).unwrap() as u8;
            let c;
            let b_plus_carry = b + carry;
            if a >= b_plus_carry {
                c = a - b_plus_carry;
                carry = 0;
            } else {
                c = a + 10 - b_plus_carry;
                carry = 1;
            }
            diff_str.push_str(&c.to_string());
        }
        if carry > 0 {
            panic!("Negative number wasn't caught in comparison!");
        }
        
        let mut n_str = String::new();
        let mut all_zeros = true;
        for ch in diff_str.chars().rev() {
            if ch != '0' {
                all_zeros = false;
            }
            if !all_zeros {
                n_str.push(ch);
            }
        }
        StrInteger { n_str }
    }

    /// Returns a tuple:
    /// (Division, Module)
    pub fn over(&self, other: &StrInteger) -> (StrInteger, StrInteger) {
        let mut remainder = self.clone();
        let mut division = StrInteger::zero();
        while remainder.ge(other) {
            remainder = remainder.minus(other);
            division = division.plus(&StrInteger::one());
        }
        (division, remainder)
    }
}


use std::cmp::Ordering;

impl PartialOrd for StrInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.n_str.len() < other.n_str.len() {
            return Some(Ordering::Less);
        } else if self.n_str.len() > other.n_str.len() {
            return Some(Ordering::Greater);
        } else {
            let self_digits = self.n_str.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8);
            let other_digits = other.n_str.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8);
            for (a, b) in self_digits.zip(other_digits) {
                if a < b {
                    return Some(Ordering::Less);
                } else if a > b {
                    return Some(Ordering::Greater);
                }
            }
        }
        // This should never be reached
        None
    }
}

use std::ops::{
    Add, AddAssign,
    Mul, MulAssign,
    Sub, SubAssign,
    Div, DivAssign,
    Rem, RemAssign
};

impl Add for StrInteger {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl AddAssign for StrInteger {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}

impl Mul for StrInteger {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}

impl MulAssign for StrInteger {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.times(&rhs);
    }
}

impl Sub for StrInteger {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl SubAssign for StrInteger {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(&rhs);
    }
}

impl Div for StrInteger {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (res, _) = self.over(&rhs);
        res
    }
}

impl DivAssign for StrInteger {
    fn div_assign(&mut self, rhs: Self) {
        let (res, _) = self.over(&rhs);
        *self = res;
    }
}

impl Rem for StrInteger {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let (_, res) = self.over(&rhs);
        res
    }
}

impl RemAssign for StrInteger {
    fn rem_assign(&mut self, rhs: Self) {
        let (_, res) = self.over(&rhs);
        *self = res;        
    }
}
