// Misc. Functions

use std::collections::HashMap;

pub fn char_to_u8(c: char) -> u8 {
    c as u8 - 48
}


pub fn u8_to_char(n: u8) -> char {
    (n + 48) as char
}


pub fn ordinal_number(n: usize) -> String {
    let ending = match n % 10 {
        1 => "st",
        2 => "nd",
        _ => "th",
    };

    format!("{}{}", &n, ending)
}


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


pub fn digits_of<T: std::fmt::Display>(n: T) -> Vec<u8> {
    let digits_str = n.to_string();
    let mut digits_vec = Vec::new();
    for ch in digits_str.chars() {
        digits_vec.push(char_to_u8(ch));
    }

    digits_vec
}


pub fn is_palindromic<T: std::fmt::Display>(x: T) -> bool {
    let digits = digits_of(x);
    let mut l_idx = 0;
    let mut r_idx = digits.len() - 1;
    while l_idx < r_idx {
        if digits[l_idx] != digits[r_idx] {
            return false;
        }
        l_idx += 1;
        r_idx -= 1;
    }

    true
}


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
