use crate::misc::ToFromChar;
use std::fmt::Write;

/// Struct of N digits, taking 1byte each.
///
/// If a bigger number is needed, use StrArray
///
/// Also implements the following operators:
///
/// +, +=, -, -=, *, *=, /, /=
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArrInteger<const N: usize> {
    pub digits: [u8; N],
}

impl<const N: usize> Default for ArrInteger<N> {
    fn default() -> Self {
        ArrInteger { digits: [0; N] }
    }
}

impl<const N: usize> ArrInteger<N> {
    /// Will return a number with value of 0;
    pub fn new() -> ArrInteger<N> {
        ArrInteger { digits: [0; N] }
    }

    /// Parse from any integer or string of digits.
    ///
    /// Will panic if ther are more than N digits on the input
    pub fn from<T: std::fmt::Display>(input: T) -> ArrInteger<N> {
        let i_chars = input.to_string();
        let mut i_chars = i_chars.chars().rev();
        let mut digits = [0; N];
        for idx in (0..N).rev() {
            if let Some(ch) = i_chars.next() {
                digits[idx] = u8::from_char(ch);
            } else {
                break;
            }
        }
        ArrInteger { digits }
    }

    /// Make a number from a slice of digits ( have to be u8 )
    ///
    /// Will panic if more than N digits are given
    pub fn from_digits(input: &[u8]) -> ArrInteger<N> {
        let mut digits = [0; N];
        if input.len() > N {
            panic!(
                "Attempting to create a {} digit number from a slice of {} digits\n{:?}",
                N,
                input.len(),
                &input
            );
        }
        input
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, &digit)| (N - idx - 1, digit))
            .for_each(|(idx, digit)| {
                digits[idx] = digit;
            });
        ArrInteger { digits }
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    pub fn pow(&self, exp: u32) -> ArrInteger<N> {
        if exp == 0 {
            return ArrInteger::<N>::from(1);
        }
        if exp == 1 {
            return *self;
        }
        let self_squared = *self * *self;
        match exp % 2 {
            0 => self_squared.pow(exp / 2),
            _ => *self * self_squared.pow((exp - 1) / 2),
        }
    }
}

impl<const N: usize> std::fmt::Display for ArrInteger<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut all_zeros = true;
        for &digit in self.digits.iter() {
            if all_zeros && digit != 0 {
                all_zeros = false;
            }
            if !all_zeros {
                write!(&mut s, "{}", digit)?;
            }
        }
        if all_zeros {
            s.push('0');
        }
        write!(f, "{}", s)
    }
}

mod operators;

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let n_int = ArrInteger::<9>::from(123456789);
        let n_str = ArrInteger::<9>::from("123456789");
        let n_arr = ArrInteger::<9>::from_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(n_int, n_str);
        assert_eq!(n_int, n_arr);
        assert_eq!(n_str, n_arr);
    }

    #[test]
    fn operators() {
        let a = ArrInteger::<10>::from(562319);
        let b = ArrInteger::<10>::from(2349);
        assert!(a != b);
        assert!(a > b);
        assert_eq!(ArrInteger::from(564668), a + b);
        assert_eq!(ArrInteger::from(1320887331), a * b);
        assert_eq!(ArrInteger::from(559970), a - b);
        assert_eq!(ArrInteger::from(239), a / b);
        assert_eq!(ArrInteger::from(908), a % b);
        let mut x = ArrInteger::from(562319);
        x += b;
        assert_eq!(ArrInteger::from(564668), x);
        let mut x = ArrInteger::from(562319);
        x *= b;
        assert_eq!(ArrInteger::from(1320887331), x);
        let mut x = ArrInteger::from(562319);
        x -= b;
        assert_eq!(ArrInteger::from(559970), x);
        let mut x = ArrInteger::from(562319);
        x /= b;
        assert_eq!(ArrInteger::from(239), x);
        let mut x = ArrInteger::from(562319);
        x %= b;
        assert_eq!(ArrInteger::from(908), x);
    }

    #[test]
    fn pow() {
        let base: u32 = 7;
        let exp = 11;
        // 1 977 326 743
        assert_eq!(
            ArrInteger::<10>::from(base.pow(exp)),
            ArrInteger::<10>::from(base).pow(exp)
        );
    }

    #[test]
    fn fmt_display() {
        let a = ArrInteger::<10>::from(562319);
        let b = ArrInteger::<10>::from(2349);
        let z = ArrInteger::<10>::from(0);

        assert_eq!("562319", a.to_string());
        assert_eq!("2349", b.to_string());
        assert_eq!("0", z.to_string());
    }
}
