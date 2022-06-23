//! Misc functions

use std::collections::HashMap;


/// takes a digit char, and returns it as an u8
pub fn char_to_u8(c: char) -> u8 {
    c as u8 - 48
}


/// takes a digit u8, and returns it as a char
pub fn u8_to_char(n: u8) -> char {
    (n + 48) as char
}


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
        digits_vec.push(char_to_u8(ch));
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
        if n % d == 0 {
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
            return format!(
                "{} thousand",
                written_out(thousands)
            );
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
            return format!(
                "{} hundred",
                cardinals[&hundreds]
            );
        }
    }

    // Tens and Units
    if n > 20 {
        let units = n % 10;
        let tens = cardinals[&((n / 10) * 10)];
        if units > 0 {
            return format!("{}-{}", &tens, &cardinals[&units]);
        } else {
            return format!("{}", &tens);
        }
    } else {
        return format!("{}", &cardinals[&n]);
    }
}


////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::misc;

    #[test]
    fn u8_to_char() {
        assert_eq!('8', misc::u8_to_char(8));
        assert_eq!('0', misc::u8_to_char(0));
    }

    #[test]
    fn char_to_u8() {
        assert_eq!(8, misc::char_to_u8('8'));
        assert_eq!(0, misc::char_to_u8('0'));
    }

    #[test]
    fn ordinal_number() {
        assert_eq!(String::from("111st"), misc::ordinal_number(111));
        assert_eq!(String::from("222nd"), misc::ordinal_number(222));
        assert_eq!(String::from("333th"), misc::ordinal_number(333));
    }

    #[test]
    fn fibonacci_term() {
        assert_eq!(8, misc::fibonacci_term(5), "Testing 'fibonacci_term({})'", 5);
        assert_eq!(89, misc::fibonacci_term(10), "Testing 'fibonacci_term({})'", 10);
    }

    #[test]
    fn digits_of() {
        assert_eq!(vec![1, 2, 3, 4, 5], misc::digits_of(12345));
        assert_eq!(vec![1, 0, 1], misc::digits_of(00101));
    }

    #[test]
    fn palindromic() {
        assert_eq!(true, misc::is_palindromic(&012321));
        assert_eq!(false, misc::is_palindromic(&12301));
        assert_eq!(true, misc::is_palindromic(&"tacocat"));
        assert_eq!(false, misc::is_palindromic(&"tacoCat"));
    }

    #[test]
    fn proper_divisors_of() {
        assert_eq!(
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110],
            misc::proper_divisors_of(220)
        );
        assert_eq!(
            vec![1, 2, 4, 71, 142],
            misc::proper_divisors_of(284)
        );
    }

    #[test]
    fn written_out() {
        assert_eq!(
            String::from("one hundred and seventy thousand eight hundred and eighty-three"),
            misc::written_out(170_883)
        );
        assert_eq!(
            String::from("nine hundred and eighty-seven thousand six hundred and fifty-four"),
            misc::written_out(987_654)
        )
    }
}
