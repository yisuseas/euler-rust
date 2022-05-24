// Misc. Functions

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
