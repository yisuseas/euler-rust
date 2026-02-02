//! Copy of [bignum](https://github.com/rust-lang/rust/blob/main/library/core/src/num/bignum.rs)

/// Table of powers of 5 representable in digits. Specifically, the largest
/// {u8, u16, u32, u64} value that's a power of five, plus the corresponding
/// exponent. Used in `mul_pow5`.
const SMALL_POW5: [(u64, usize); 4] = [
    (125, 3),
    (15_625, 6),
    (1_220_703_125, 13),
    (7_450_580_596_923_828_125, 27),
];

/// Stack-allocated arbitrary-precision (up to certain limit) integer.
///
/// This is backed by a fixed-size array of `u64` ("digit").
/// While the array is not very large (128 bytes),
/// copying it recklessly may result in the performance hit.
///
/// All operations available to bignums panic in the case of overflows.
/// The caller is responsible to use large enough bignum types.
#[derive(Clone, Copy)]
pub struct U1024 {
    size: usize,
    base: [u64; 16],
}

impl U1024 {
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = U1024 {
        size: 16,
        base: [u64::MAX; 16],
    };

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = U1024 {
        size: 0,
        base: [u64::MIN; 16],
    };

    /// The number `1` represented by this integer type.
    pub const ONE: Self = U1024 {
        size: 1,
        base: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    /// Makes a `U1024` from one digit.
    pub fn from_small(v: u64) -> U1024 {
        let mut base = [0; 16];
        base[0] = v;
        U1024 { size: 1, base }
    }

    /// Makes a `U1024` from `u128` value.
    pub fn from_u128(mut v: u128) -> U1024 {
        let mut base = [0; 16];
        let mut sz = 0;
        while v > 0 {
            base[sz] = v as u64;
            v >>= <u64>::BITS;
            sz += 1;
        }
        U1024 { size: sz, base }
    }

    /// Returns the internal digits as a slice `[a, b, c, ...]` such that the
    /// numeric value is `a + b * 2^64 + c * 2^(2*64) + ...`
    pub fn digits(&self) -> &[u64] {
        &self.base[..self.size]
    }

    /// Returns the `i`-th bit where bit 0 is the least significant one.
    /// In other words, the bit with weight `2^i`.
    pub fn get_bit(&self, i: usize) -> u8 {
        let digitbits = <u64>::BITS as usize;
        let d = i / digitbits;
        let b = i % digitbits;
        ((self.base[d] >> b) & 1) as u8
    }

    /// Returns `true` if the U1024 is zero.
    pub fn is_zero(&self) -> bool {
        self.digits().iter().all(|&v| v == 0)
    }

    /// Returns the number of bits necessary to represent this value. Note that
    /// zero is considered to need 0 bits.
    pub fn bit_length(&self) -> usize {
        let digitbits = <u64>::BITS as usize;
        let digits = self.digits();
        let msd = digits.iter().rposition(|&x| x != 0);
        match msd {
            Some(msd) => msd * digitbits + digits[msd].ilog2() as usize + 1,
            _ => 0,
        }
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    pub fn pow(&self, exp: u32) -> U1024 {
        match exp {
            0 => U1024::ONE,
            1 => *self,
            _ => {
                let self_squared = (*self) * (*self);
                if exp.is_multiple_of(2) {
                    self_squared.pow(exp / 2)
                } else {
                    (*self) * self_squared.pow((exp - 1) / 2)
                }
            }
        }
    }
}

impl std::cmp::PartialEq for U1024 {
    fn eq(&self, other: &U1024) -> bool {
        self.base[..] == other.base[..]
    }
}

impl std::cmp::Eq for U1024 {}

impl std::cmp::PartialOrd for U1024 {
    fn partial_cmp(&self, other: &U1024) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for U1024 {
    fn cmp(&self, other: &U1024) -> std::cmp::Ordering {
        use std::cmp::max;
        let sz = max(self.size, other.size);
        let lhs = self.base[..sz].iter().cloned().rev();
        let rhs = other.base[..sz].iter().cloned().rev();
        lhs.cmp(rhs)
    }
}

impl std::fmt::Display for U1024 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sz = if self.size < 1 { 1 } else { self.size };

        use super::StrInteger;
        let mut sum = StrInteger::from(self.base[0]);
        for (idx, v) in self.base.iter().take(sz).enumerate().skip(1) {
            let num = StrInteger::from(v);
            let factor = StrInteger::from(2).pow((idx as u32) * <u64>::BITS);
            sum += num * factor;
        }
        write!(f, "{}", sum)?;

        Ok(())
    }
}

impl std::fmt::Debug for U1024 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sz = if self.size < 1 { 1 } else { self.size };
        let digitlen = <u64>::BITS as usize / 4;

        write!(f, "{:#x}", self.base[sz - 1])?;
        for &v in self.base[..sz - 1].iter().rev() {
            write!(f, "_{:01$x}", v, digitlen)?;
        }
        Ok(())
    }
}

impl std::hash::Hash for U1024 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.base.hash(state);
    }
}

mod operators;

#[cfg(test)]
mod tests;
