use super::*;

/// Arithmetic operations required by bignums.
trait FullOps: Sized {
    /// Returns `(carry', v')` such that `carry' * 2^W + v' = self * other + other2 + carry`,
    /// where `W` is the number of bits in `Self`.
    fn full_mul_add(
        self,
        other: Self,
        other2: Self,
        carry: Self,
    ) -> (Self /* carry */, Self);

    /// Returns `(quo, rem)` such that `borrow * 2^W + self = quo * other + rem`
    /// and `0 <= rem < other`, where `W` is the number of bits in `Self`.
    fn full_div_rem(
        self,
        other: Self,
        borrow: Self,
    ) -> (Self /* quotient */, Self /* remainder */);
}

impl FullOps for u64 {
    fn full_mul_add(
        self,
        other: Self,
        other2: Self,
        carry: Self,
    ) -> (Self /* carry */, Self) {
        // This cannot overflow;
        // the output is between `0` and `2^nbits * (2^nbits - 1)`.
        let (lo, hi) = self.carrying_mul_add(other, other2, carry);
        (hi, lo)
    }

    fn full_div_rem(
        self,
        other: Self,
        borrow: Self,
    ) -> (Self /* quotient */, Self /* remainder */) {
        debug_assert!(borrow < other);
        // This cannot overflow; the output is between `0` and `other * (2^nbits - 1)`.
        let lhs = ((borrow as u128) << <u64>::BITS) | (self as u128);
        let rhs = other as u128;
        ((lhs / rhs) as u64, (lhs % rhs) as u64)
    }
}

impl U1024 {
    /// Adds `other` to itself and returns its own mutable reference.
    pub fn plus<'a>(&'a mut self, other: &U1024) -> &'a mut U1024 {
        use std::{cmp, iter};

        let mut sz = cmp::max(self.size, other.size);
        let mut carry = false;
        for (a, b) in iter::zip(&mut self.base[..sz], &other.base[..sz]) {
            let (v, c) = (*a).carrying_add(*b, carry);
            *a = v;
            carry = c;
        }
        if carry {
            self.base[sz] = 1;
            sz += 1;
        }
        self.size = sz;
        self
    }

    /// Adds a digit-sized `other` to itself and returns its own
    /// mutable reference.
    pub fn plus_small(&mut self, other: u64) -> &mut U1024 {
        let (v, mut carry) = self.base[0].carrying_add(other, false);
        self.base[0] = v;
        let mut i = 1;
        while carry {
            let (v, c) = self.base[i].carrying_add(0, carry);
            self.base[i] = v;
            carry = c;
            i += 1;
        }
        if i > self.size {
            self.size = i;
        }
        self
    }

    /// Substracts other from itself and returns its own mutable reference.
    pub fn minus<'a>(&'a mut self, other: &U1024) -> &'a mut U1024 {
        use std::{cmp, iter};

        let sz = cmp::max(self.size, other.size);
        let mut noborrow = true;
        for (a, b) in iter::zip(&mut self.base[..sz], &other.base[..sz]) {
            let (v, c) = (*a).carrying_add(!*b, noborrow);
            *a = v;
            noborrow = c;
        }
        assert!(noborrow);
        self.size = sz;
        self
    }

    /// Multiplies itself by a digit-sized `other` and returns its own
    /// mutable reference.
    pub fn mul_small(&mut self, other: u64) -> &mut U1024 {
        let mut sz = self.size;
        let mut carry = 0;
        for a in &mut self.base[..sz] {
            let (v, c) = (*a).carrying_mul(other, carry);
            *a = v;
            carry = c;
        }
        if carry > 0 {
            self.base[sz] = carry;
            sz += 1;
        }
        self.size = sz;
        self
    }

    /// Multiplies itself by `2^bits` and returns its own mutable reference.
    pub fn mul_pow2(&mut self, bits: usize) -> &mut U1024 {
        let digitbits = <u64>::BITS as usize;
        let digits = bits / digitbits;
        let bits = bits % digitbits;

        assert!(digits < 16);
        debug_assert!(self.base[16 - digits..].iter().all(|&v| v == 0));
        debug_assert!(
            bits == 0
                || (self.base[16 - digits - 1] >> (digitbits - bits)) == 0
        );

        for i in (0..self.size).rev() {
            self.base[i + digits] = self.base[i];
        }
        for i in 0..digits {
            self.base[i] = 0;
        }

        let mut sz = self.size + digits;
        if bits > 0 {
            let last = sz;
            let overflow = self.base[last - 1] >> (digitbits - bits);
            if overflow > 0 {
                self.base[last] = overflow;
                sz += 1;
            }
            for i in (digits + 1..last).rev() {
                self.base[i] = (self.base[i] << bits)
                    | (self.base[i - 1] >> (digitbits - bits));
            }
            self.base[digits] <<= bits;
        }

        self.size = sz;
        self
    }

    /// Multiplies itself by `5^exp` and returns its own mutable reference.
    pub fn mul_pow5(&mut self, mut exp: usize) -> &mut U1024 {
        let table_index = size_of::<u64>().trailing_zeros() as usize;
        let (small_power, small_exp) = SMALL_POW5[table_index];

        while exp >= small_exp {
            self.mul_small(small_power);
            exp -= small_exp;
        }

        let mut rest_power = 1;
        for _ in 0..exp {
            rest_power *= 5;
        }
        self.mul_small(rest_power);

        self
    }

    /// Multiplies itself by a number described by
    /// `other[0] + other[1] * 2^64 + other[2] * 2^(2*64) + ...`
    /// and returns its own mutable reference.
    pub fn mul_digits<'a>(&'a mut self, other: &[u64]) -> &'a mut U1024 {
        fn mul_inner(ret: &mut [u64; 16], aa: &[u64], bb: &[u64]) -> usize {
            let mut retsz = 0;
            for (i, &a) in aa.iter().enumerate() {
                if a == 0 {
                    continue;
                }
                let mut sz = bb.len();
                let mut carry = 0;
                for (j, &b) in bb.iter().enumerate() {
                    let (c, v) = a.full_mul_add(b, ret[i + j], carry);
                    ret[i + j] = v;
                    carry = c;
                }
                if carry > 0 {
                    ret[i + sz] = carry;
                    sz += 1;
                }
                if retsz < i + sz {
                    retsz = i + sz;
                }
            }
            retsz
        }

        let mut ret = [0; 16];
        let retsz = if self.size < other.len() {
            mul_inner(&mut ret, self.digits(), other)
        } else {
            mul_inner(&mut ret, other, self.digits())
        };
        self.base = ret;
        self.size = retsz;
        self
    }

    /// Divides itself by a digit-sized `other` and returns its own
    /// mutable reference *and* the remainder.
    pub fn div_rem_small(&mut self, other: u64) -> (&mut U1024, u64) {
        assert!(other > 0);

        let sz = self.size;
        let mut borrow = 0;
        for a in self.base[..sz].iter_mut().rev() {
            let (q, r) = (*a).full_div_rem(other, borrow);
            *a = q;
            borrow = r;
        }
        (self, borrow)
    }
}

impl std::ops::Add for U1024 {
    type Output = U1024;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.plus(&rhs);
        res
    }
}

impl std::ops::AddAssign for U1024 {
    fn add_assign(&mut self, rhs: Self) {
        self.plus(&rhs);
    }
}

impl std::ops::Sub for U1024 {
    type Output = U1024;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.minus(&rhs);
        res
    }
}

impl std::ops::SubAssign for U1024 {
    fn sub_assign(&mut self, rhs: Self) {
        self.minus(&rhs);
    }
}

impl std::ops::Mul for U1024 {
    type Output = U1024;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.mul_digits(rhs.digits());
        res
    }
}

impl std::ops::MulAssign for U1024 {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_digits(rhs.digits());
    }
}

impl std::ops::Div for U1024 {
    type Output = U1024;
    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.size == 1);
        self.clone().div_rem_small(rhs.base[0]).0.to_owned()
    }
}

impl std::ops::DivAssign for U1024 {
    fn div_assign(&mut self, rhs: Self) {
        assert!(rhs.size == 1);
        self.div_rem_small(rhs.base[0]);
    }
}

impl std::ops::Rem for U1024 {
    type Output = U1024;
    fn rem(self, rhs: Self) -> Self::Output {
        assert!(rhs.size == 1);
        let mut cl = self;
        let (_, res) = cl.div_rem_small(rhs.base[0]);
        U1024::from_small(res)
    }
}

impl std::ops::RemAssign for U1024 {
    fn rem_assign(&mut self, rhs: Self) {
        assert!(rhs.size == 1);
        let (_, res) = self.div_rem_small(rhs.base[0]);
        *self = U1024::from_small(res);
    }
}
