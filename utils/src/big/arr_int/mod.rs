use crate::misc;

/// Struct of 200 digits, taking 1byte each.
///
/// If a bigger number is needed, use StrArray
///
/// Also implements the following operators:
///
/// +, +=, -, -=, *, *=, /, /=
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArrInteger {
  pub digits: [u8; 200],
}

impl ArrInteger {
  /// Will return a number with value of 0;
  pub fn new() -> ArrInteger {
    ArrInteger { digits: [0; 200] }
  }

  /// Parse from any integer or string of digits.
  ///
  /// Will panic if ther are more than 200 digits on the input
  pub fn from<T: std::fmt::Display>(input: T) -> ArrInteger {
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
    ArrInteger { digits }
  }

  /// Make a number from a slice of digits ( have to be u8 )
  ///
  /// Will panic if more than 200 digits are given
  pub fn from_digits(input: &[u8]) -> ArrInteger {
    let mut digits = [0; 200];
    input
      .iter()
      .rev()
      .enumerate()
      .map(|(idx, &digit)| (200 - idx - 1, digit))
      .for_each(|(idx, digit)| {
        digits[idx] = digit;
      });
    ArrInteger { digits }
  }
}

impl std::fmt::Display for ArrInteger {
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
    let n_int = ArrInteger::from(123456789);
    let n_str = ArrInteger::from("123456789");
    let n_arr = ArrInteger::from_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(n_int, n_str);
    assert_eq!(n_int, n_arr);
    assert_eq!(n_str, n_arr);
  }

  #[test]
  fn operators() {
    let a = ArrInteger::from(562319);
    let b = ArrInteger::from(2349);
    assert!(a != b);
    assert!(a > b);
    assert_eq!(ArrInteger::from(564668), a + b);
    assert_eq!(ArrInteger::from(1320887331), a * b);
    assert_eq!(ArrInteger::from(559970), a - b);
    assert_eq!(ArrInteger::from(239), a / b);
    assert_eq!(ArrInteger::from(908), a % b);
  }
}
