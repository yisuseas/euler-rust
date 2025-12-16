/// Struct containing a String as a representation of a number
///
/// Since String's are handled on heap, and cannot derive the Copy Trait,
/// StrInteger will perform slower than ArrInteger.
///
/// Only use if there's a need for more than 200 digits
///
/// Also implements the following operators:
///
/// +,  +=,  -,  -=,  *,  *=,  /,  /=
///
/// Note that all of these will consume their arguments.
/// their dedicated functions might be more useful to work with references.
/// Otherwise use deep clones
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrInteger {
    n_str: String,
}

impl StrInteger {
    /// Panics if __n__ contains a character that is not a digit between 0 and 9
    pub fn from<T: std::fmt::Display>(n: T) -> StrInteger {
        let n_str = n.to_string();
        for ch in n_str.chars() {
            match ch.to_string().parse::<u8>() {
                Ok(n) if n > 10 => {
                    panic!("Parsing error: {} is not a digit", &n)
                }
                Err(e) => panic!("Parsing error: {}", &e),
                _ => (),
            }
        }
        StrInteger { n_str }
    }

    /// Panics if __input__ contains an item that is not a digit between 0 and 9
    pub fn from_digits<T: std::fmt::Display>(input: &[T]) -> StrInteger {
        let mut n_str = String::new();
        for item in input.iter() {
            match item.to_string().parse::<u8>() {
                Ok(n) if n > 10 => {
                    panic!("Parsing error: {} is not a digit", &n)
                }
                Err(e) => panic!("Parsing error: {}", &e),
                _ => n_str.push_str(&item.to_string()),
            }
        }
        StrInteger { n_str }
    }

    pub fn one() -> StrInteger {
        StrInteger {
            n_str: String::from("1"),
        }
    }

    pub fn zero() -> StrInteger {
        StrInteger {
            n_str: String::from("0"),
        }
    }

    /// The ammount of digits of the number, not to be confused with an array of them
    pub fn digits(&self) -> usize {
        self.n_str.len()
    }

    /// Raises self to the power or `exp`, using exponentiation by squaring.
    pub fn pow(&self, exp: u32) -> Self {
        if exp == 0 {
            return Self::one();
        }
        if exp == 1 {
            return self.clone();
        }
        let self_squared = self.times(self);
        match exp % 2 {
            0 => self_squared.pow(exp / 2),
            _ => self.times(&(self_squared.pow((exp - 1) / 2))),
        }
    }
}

impl std::fmt::Display for StrInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.n_str)
    }
}

mod operators;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let a = StrInteger::from(123456789);
        let b = StrInteger::from("123456789");
        let c = StrInteger::from_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let d = StrInteger::from_digits(&[
            '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ]);
        let e = StrInteger::from_digits(&[
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, d);
        assert_eq!(d, e);
        assert_eq!(e, a);
    }

    #[test]
    fn conversion() {
        let eight_u8 = 8;
        // str -> u8
        let eight_str = "8";
        assert_eq!(eight_u8, eight_str.parse::<u8>().ok().unwrap());
        // char -> u8
        let eight_char = '8';
        assert_eq!(eight_u8, eight_char.to_digit(10).unwrap() as u8);

        let twelve_u8 = 12;
        // str -> u8
        let twelve_str = "12";
        assert_eq!(twelve_u8, twelve_str.parse::<u8>().ok().unwrap());
        // [char] -> String
        let twelve_char = ['1', '2'];
        let char_arr_to_string = twelve_char.iter().collect::<String>();
        assert_eq!(twelve_str, char_arr_to_string);
        // String -> u8
        assert_eq!(twelve_u8, char_arr_to_string.parse::<u8>().ok().unwrap());
    }

    #[test]
    fn operators() {
        let a = StrInteger::from(562_319);
        let b = StrInteger::from(2349);
        assert!(a != b);
        assert!(a > b);
        assert_eq!(StrInteger::from(564_668), a.plus(&b));
        assert_eq!(StrInteger::from(564_668), a.clone() + b.clone());
        assert_eq!(StrInteger::from(1_320_887_331), a.times(&b));
        assert_eq!(StrInteger::from(1_320_887_331), a.clone() * b.clone());
        assert_eq!(StrInteger::from(559_970), a.minus(&b));
        assert_eq!(StrInteger::from(559_970), a.clone() - b.clone());
        let a_div_b = StrInteger::from(239);
        let a_mod_b = StrInteger::from(908);
        assert_eq!((a_div_b.clone(), a_mod_b.clone()), a.over(&b));
        assert_eq!((a_div_b, a_mod_b), (a.clone() / b.clone(), a % b));
    }

    #[test]
    fn pow() {
        let base: u32 = 7;
        let exp = 11;
        assert_eq!(
            StrInteger::from(base.pow(exp)),
            StrInteger::from(base).pow(exp)
        );
    }
}
