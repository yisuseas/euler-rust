use super::*;

impl BigInteger {
    fn plus(&self, other: &BigInteger) -> BigInteger {
        let mut sum_digits = [0; 200];
        let mut remainder = 0;
        for idx in (0..200).rev() {
            let partial_sum = self.digits[idx] +
                                  other.digits[idx] +
                                  remainder;
            sum_digits[idx] = partial_sum % 10;
            remainder = partial_sum / 10;
        }
        if remainder > 0 {
            panic!("Attempt to add with Overflow")
        }
        BigInteger { digits: sum_digits }
    }

    fn times(&self, other: &BigInteger) -> BigInteger {
        let mut partial_mult_vec = Vec::new();
        other.digits.iter().rev()
             .enumerate()
             .for_each(|(other_idx, &other_digit)|
        {
            let mut partial_mult = Vec::new();
            let mut remainder = 0;

            for _ in 0..other_idx {
                partial_mult.push(0);
            }

            self.digits.iter().rev()
                .for_each(|&self_digit|
            {
                remainder += other_digit * self_digit;
                partial_mult.push(remainder % 10);
                remainder /= 10;
            });

            while remainder > 0 {
                partial_mult.push(remainder % 10);
                remainder /= 10;
            }

            for _ in 0..other_idx {
                if let Some(n) = partial_mult.pop() {
                    if n != 0 {
                        panic!("Attempt to multiply with Overflow");
                    }
                }
            }

            let partial_mult: Vec<u8> = partial_mult.iter().rev()
                                                    .map(|a| *a)
                                                    .collect();
            partial_mult_vec.push(BigInteger::from_digits(&partial_mult));
        });
        let mut product = BigInteger::new();
        for big_int in partial_mult_vec {
            product = product.plus(&big_int);
        }

        product
    }

    fn minus(&self, other: &BigInteger) -> BigInteger {
        let mut digits = [0; 200];
        let mut carry = 0;
        for idx in (0..200).rev() {
            let a = self.digits[idx];
            let b = other.digits[idx];
            let c;
            if a >= (b + carry) {
                c = a - (b + carry);
                carry = 0;
            } else {
                c = a + 10 - (b + carry);
                carry = 1;
            }
            digits[idx] = c;
        }
        if carry > 0 {
            panic!("Attempt to substract with Underflow");
        }
        BigInteger { digits }
    }

    fn over(&self, other: &BigInteger) -> (BigInteger, BigInteger) {
        let mut remainder = self.clone();
        let mut division = BigInteger::new();
        let one = BigInteger::from(1);
        while remainder.ge(other) {
            remainder = remainder.minus(other);
            division = division + one;
        }
        (division, remainder)
    }
}


impl std::cmp::PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut o = std::cmp::Ordering::Equal;
        if self == other {
            return Some(o);
        }
        for idx in 0..200 {
            if self.digits[idx] > other.digits[idx] {
                o = std::cmp::Ordering::Greater;
                break;
            } else if self.digits[idx] < other.digits[idx] {
                o = std::cmp::Ordering::Less;
                break;
            }
        }
        Some(o)
    }
}

impl std::ops::Add for BigInteger {
    type Output = BigInteger;
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl std::ops::AddAssign for BigInteger {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}

impl std::ops::Mul for BigInteger {
    type Output = BigInteger;
    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}

impl std::ops::Sub for BigInteger {
    type Output = BigInteger;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl std::ops::Div for BigInteger {
    type Output = BigInteger;
    fn div(self, rhs: Self) -> Self::Output {
        let (result, _) = self.over(&rhs);
        result
    }
}

impl std::ops::Rem for BigInteger {
    type Output = BigInteger;
    fn rem(self, rhs: Self) -> Self::Output {
        let (_, result) = self.over(&rhs);
        result
    }
}
