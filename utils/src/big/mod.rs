use crate::misc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BigInteger {
    pub digits: [u8; 200]
}


impl BigInteger {
    pub fn new() -> BigInteger {
        BigInteger { digits: [0; 200] }
    }

    pub fn from<T: std::fmt::Display>(input: T) -> BigInteger {
        let i_chars = input.to_string();
        let mut i_chars = i_chars.chars().rev();
        let mut digits = [0; 200];
        for idx in (0..200).rev() {
            if let Some(ch) = i_chars.next() {
                digits[idx] = misc::char_to_u8(ch);
            } else {
                break;
            }
        }
        BigInteger { digits }
    }

    pub fn from_digits(input: &[u8]) -> BigInteger {
        let mut digits = [0; 200];
        input.iter().rev()
            .enumerate()
            .map(|(idx, &digit)| (200 - idx - 1, digit))
            .for_each(|(idx, digit)|
        {
            digits[idx] = digit;
        });
        BigInteger { digits }
    }
}


impl std::fmt::Display for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut all_zeros = true;
        for &digit in self.digits.iter() {
            if all_zeros {
                if digit != 0 {
                    all_zeros = false;
                }
            }
            if !all_zeros {
                s.push_str(&format!("{}", digit));
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
        let n_int = BigInteger::from(123456789);
        let n_str = BigInteger::from("123456789");
        let n_arr = BigInteger::from_digits(&[1,2,3,4,5,6,7,8,9]);
        assert_eq!(n_int, n_str);
        assert_eq!(n_int, n_arr);
        assert_eq!(n_str, n_arr);
    }

    #[test]
    fn operators() {
        let a = BigInteger::from(562319);
        let b = BigInteger::from(2349);
        assert!(a != b);
        assert!(a > b);
        assert_eq!(BigInteger::from(564668), a + b);
        assert_eq!(BigInteger::from(1320887331), a * b);
        assert_eq!(BigInteger::from(559970), a - b);
        assert_eq!(BigInteger::from(239), a / b);
        assert_eq!(BigInteger::from(908), a % b);
    }
}

