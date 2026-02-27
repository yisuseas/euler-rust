//! Misc functions

use std::collections::HashMap;

pub trait ToFromChar {
    /// takes a digit unsigned integer, and returns it as a char.
    fn to_char(&self) -> char;
    /// Takes a digit char, and returns it as an unsigned integer.
    fn from_char(ch: char) -> Self;
}

macro_rules! impl_to_from_char {
    ( $( $ty:ty ),* ) => {
        $(
            impl ToFromChar for $ty {
                fn to_char(&self) -> char {
                    (*self as u8 + 48) as char
                }
                fn from_char(ch: char) -> $ty {
                    (ch as u8 - 48) as $ty
                }
            }
        )*
    };
}

impl_to_from_char!(u8, u16, u32, u64, u128);

/// The ordinal representation of a given integer
pub fn ordinal_number(n: usize) -> String {
    let ending = match n % 10 {
        1 => "st",
        2 => "nd",
        _ => "th",
    };

    format!("{}{}", &n, ending)
}

/// The nth term of the fibonacci sequence
/// Given:
/// 1st = 1,
/// 2nd = 2,
pub fn fibonacci_term(term: u64) -> u64 {
    if term < 1 {
        return 0;
    }
    match term {
        1 => 1,
        2 => 2,
        n => fibonacci_term(n - 1) + fibonacci_term(n - 2),
    }
}

/// Returns a vector of the digits of a number, as u8 values
pub fn digits_of<T: std::fmt::Display>(n: T) -> Vec<u8> {
    let digits_str = n.to_string();
    let mut digits_vec = Vec::new();
    for ch in digits_str.chars() {
        digits_vec.push(u8::from_char(ch));
    }

    digits_vec
}

/// Returns true if the given input is palindromic.
pub fn is_palindromic<T: std::fmt::Display>(input: &T) -> bool {
    let s = input.to_string();
    let mut regular = s.chars();
    let mut inverse = s.chars().rev();
    for _ in 0..s.len() / 2 {
        if regular.next() != inverse.next() {
            return false;
        }
    }
    true
}

/// Returns a vector of the proper divisors of a given integer
pub fn proper_divisors_of(n: usize) -> Vec<usize> {
    // Find half of the divisors
    let mut first_half = Vec::new();
    for d in 2..((n as f64).sqrt() as usize + 1) {
        if n.is_multiple_of(d) {
            first_half.push(d);
        }
    }
    // Divide n by each of them to get the other half
    let mut second_half = Vec::new();
    for &d in first_half.iter().rev() {
        if n / d != d {
            second_half.push(n / d);
        }
    }
    // Join them
    [vec![1], first_half, second_half].concat()
}

/// Returns a Vector of Permutations of the members
pub fn permutations<T: Copy>(members: &[T]) -> Vec<Vec<T>> {
    if members.len() == 1 {
        return vec![vec![members[0]]];
    }
    let mut perms = Vec::new();
    for idx in 0..members.len() {
        let first = members[idx];
        let remainder: Vec<T> = members
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != idx)
            .map(|(_, &item)| item)
            .collect();
        for p in permutations(&remainder) {
            perms.push([vec![first], p].concat());
        }
    }
    perms
}

/// Will return a String containing the written representation of a nuymber.
///
/// Based on british usage.
pub fn written_out(n: usize) -> String {
    let cardinals: HashMap<usize, &str> = HashMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]);

    // Thousands
    if n >= 1_000 {
        let thousands = n / 1_000;
        let remainder = n % 1_000;

        if remainder > 0 {
            return format!(
                "{} thousand {}",
                written_out(thousands),
                written_out(remainder)
            );
        } else {
            return format!("{} thousand", written_out(thousands));
        }
    }

    // Hundreds
    if n >= 100 {
        let hundreds = n / 100;
        let remainder = n % 100;

        if remainder > 0 {
            return format!(
                "{} hundred and {}",
                cardinals[&hundreds],
                written_out(remainder)
            );
        } else {
            return format!("{} hundred", cardinals[&hundreds]);
        }
    }

    // Tens and Units
    if n > 20 {
        let units = n % 10;
        let tens = cardinals[&((n / 10) * 10)];
        if units > 0 {
            format!("{}-{}", &tens, &cardinals[&units])
        } else {
            tens.to_string()
        }
    } else {
        cardinals[&n].to_string()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::misc;

    #[test]
    fn uint_to_char() {
        use crate::misc::ToFromChar;
        assert_eq!('8', 8u8.to_char());
        assert_eq!('8', 8u16.to_char());
        assert_eq!('8', 8u32.to_char());
        assert_eq!('8', 8u64.to_char());
        assert_eq!('8', 8u128.to_char());
        assert_eq!('0', 0u8.to_char());
        assert_eq!('0', 0u16.to_char());
        assert_eq!('0', 0u32.to_char());
        assert_eq!('0', 0u64.to_char());
        assert_eq!('0', 0u128.to_char());
    }

    #[test]
    fn char_to_uint() {
        use crate::misc::ToFromChar;
        assert_eq!(8u8, u8::from_char('8'));
        assert_eq!(8u16, u16::from_char('8'));
        assert_eq!(8u32, u32::from_char('8'));
        assert_eq!(8u64, u64::from_char('8'));
        assert_eq!(8u128, u128::from_char('8'));
        assert_eq!(0u8, u8::from_char('0'));
        assert_eq!(0u16, u16::from_char('0'));
        assert_eq!(0u32, u32::from_char('0'));
        assert_eq!(0u64, u64::from_char('0'));
        assert_eq!(0u128, u128::from_char('0'));
    }

    #[test]
    fn ordinal_number() {
        assert_eq!(String::from("111st"), misc::ordinal_number(111));
        assert_eq!(String::from("222nd"), misc::ordinal_number(222));
        assert_eq!(String::from("333th"), misc::ordinal_number(333));
    }

    #[test]
    fn fibonacci_term() {
        assert_eq!(
            8,
            misc::fibonacci_term(5),
            "Testing 'fibonacci_term({})'",
            5
        );
        assert_eq!(
            89,
            misc::fibonacci_term(10),
            "Testing 'fibonacci_term({})'",
            10
        );
    }

    #[test]
    fn digits_of() {
        assert_eq!(vec![1, 2, 3, 4, 5], misc::digits_of(12345));
        assert_eq!(vec![1, 0, 1], misc::digits_of(101));
    }

    #[test]
    fn palindromic() {
        assert!(misc::is_palindromic(&12321));
        assert!(!misc::is_palindromic(&12301));
        assert!(misc::is_palindromic(&"tacocat"));
        assert!(!misc::is_palindromic(&"tacoCat"));
    }

    #[test]
    fn proper_divisors_of() {
        assert_eq!(
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110],
            misc::proper_divisors_of(220)
        );
        assert_eq!(vec![1, 2, 4, 71, 142], misc::proper_divisors_of(284));
    }

    #[test]
    fn permutations() {
        let a = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(a, misc::permutations(&[1, 2, 3]))
    }

    #[test]
    fn written_out() {
        assert_eq!(
      String::from(
        "one hundred and seventy thousand eight hundred and eighty-three"
      ),
      misc::written_out(170_883)
    );
        assert_eq!(
      String::from(
        "nine hundred and eighty-seven thousand six hundred and fifty-four"
      ),
      misc::written_out(987_654)
    )
    }
}
