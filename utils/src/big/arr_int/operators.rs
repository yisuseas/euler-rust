use super::*;

impl<const N: usize> ArrInteger<N> {
    fn plus(&self, other: &ArrInteger<N>) -> ArrInteger<N> {
        let mut sum_digits = [0; N];
        let mut remainder = 0;
        for idx in (0..N).rev() {
            let partial_sum = self.digits[idx] + other.digits[idx] + remainder;
            sum_digits[idx] = partial_sum % 10;
            remainder = partial_sum / 10;
        }
        if remainder > 0 {
            panic!("Attempt to add with Overflow")
        }
        ArrInteger { digits: sum_digits }
    }

    fn times(&self, multiplier: &ArrInteger<N>) -> ArrInteger<N> {
        let mut partial_mult_arr = [[0; N]; N];

        multiplier.digits.iter().enumerate().rev().for_each(
            |(multiplier_idx, &multiplier_digit)| {
                let mut partial_mult = [0; N];
                let mut remainder = 0;

                // 'offset' porpousefully counting backwards
                for (offset, &multiplicand_digit) in
                    self.digits.iter().rev().enumerate()
                {
                    if offset > multiplier_idx {
                        break;
                    }
                    remainder += multiplier_digit * multiplicand_digit;
                    partial_mult[multiplier_idx - offset] = remainder % 10;
                    remainder /= 10;
                }

                if remainder > 0 {
                    panic!("Attempt to multiply with Overflow");
                }

                partial_mult_arr[multiplier_idx] = partial_mult;
            },
        );

        // Add up all the partial multiplications
        let mut sum_digits = [0; N];
        let mut remainder = 0;

        for idx in (0..N).rev() {
            remainder += partial_mult_arr
                .iter()
                .map(|partial_mult| partial_mult[idx] as u64)
                .sum::<u64>();
            sum_digits[idx] = (remainder % 10) as u8;
            remainder /= 10;
        }

        ArrInteger { digits: sum_digits }
    }

    fn minus(&self, other: &ArrInteger<N>) -> ArrInteger<N> {
        let mut digits = [0; N];
        let mut carry = 0;
        for idx in (0..N).rev() {
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
        ArrInteger { digits }
    }

    fn over(&self, other: &ArrInteger<N>) -> (ArrInteger<N>, ArrInteger<N>) {
        // let mut remainder = self.clone();
        let mut remainder = *self;
        let mut division = ArrInteger::<N>::new();
        let one = ArrInteger::<N>::from(1);
        while remainder.ge(other) {
            remainder = remainder.minus(other);
            // division = division + one;
            division = division.plus(&one);
        }
        (division, remainder)
    }
}

impl<const N: usize> std::cmp::PartialOrd for ArrInteger<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering = std::cmp::Ordering::Equal;
        if self == other {
            return Some(ordering);
        }
        for idx in 0..N {
            match self.digits[idx].cmp(&(other.digits[idx])) {
                std::cmp::Ordering::Equal => {}
                o => {
                    ordering = o;
                    break;
                }
            }
            // if self.digits[idx] > other.digits[idx] {
            //     ordering = std::cmp::Ordering::Greater;
            //     break;
            // } else if self.digits[idx] < other.digits[idx] {
            //     ordering = std::cmp::Ordering::Less;
            //     break;
            // }
        }
        Some(ordering)
    }
}

impl<const N: usize> std::ops::Add for ArrInteger<N> {
    type Output = ArrInteger<N>;
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl<const N: usize> std::ops::AddAssign for ArrInteger<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}

impl<const N: usize> std::ops::Mul for ArrInteger<N> {
    type Output = ArrInteger<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.times(&rhs)
    }
}

impl<const N: usize> std::ops::MulAssign for ArrInteger<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.times(&rhs)
    }
}

impl<const N: usize> std::ops::Sub for ArrInteger<N> {
    type Output = ArrInteger<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl<const N: usize> std::ops::SubAssign for ArrInteger<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(&rhs)
    }
}

impl<const N: usize> std::ops::Div for ArrInteger<N> {
    type Output = ArrInteger<N>;
    fn div(self, rhs: Self) -> Self::Output {
        let (result, _) = self.over(&rhs);
        result
    }
}

impl<const N: usize> std::ops::DivAssign for ArrInteger<N> {
    fn div_assign(&mut self, rhs: Self) {
        let (result, _) = self.over(&rhs);
        *self = result;
    }
}

impl<const N: usize> std::ops::Rem for ArrInteger<N> {
    type Output = ArrInteger<N>;
    fn rem(self, rhs: Self) -> Self::Output {
        let (_, result) = self.over(&rhs);
        result
    }
}

impl<const N: usize> std::ops::RemAssign for ArrInteger<N> {
    fn rem_assign(&mut self, rhs: Self) {
        let (_, result) = self.over(&rhs);
        *self = result;
    }
}
